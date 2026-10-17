/*
   OxiRush
   Copyright 2025-2026 Valentin D'Emmanuele

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

   http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

/* One operation of the asn1c code of Open5GS on one PDU, as compare.py runs it:

       asn1c OPERATION ITERATIONS HEX

   OPERATION is `decode`, the PDU decoded with every IE of its type and freed, as asn1c
   decodes, or `encode`, that PDU written again. It runs the operation ITERATIONS times
   as a warm-up, then three times ITERATIONS times, and prints {"ns": N, "allocations":
   A, "bytes": B, "held": H}: the time of a call in the fastest of the three, the
   allocations and the octets that a call asks of malloc, calloc and realloc, which this
   program counts in the place of those of the C library, and the most memory that the
   program held, in KiB. It is built with -DPROTOCOL_NGAP or
   -DPROTOCOL_S1AP against the libraries that compare.py builds of Open5GS; it holds no
   code of Open5GS. This file is the same in oxirush-ngap and oxirush-s1ap. */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#include "ogs-core.h"
#if defined(PROTOCOL_NGAP)
#include "NGAP_NGAP-PDU.h"
#define PDU_TYPE NGAP_NGAP_PDU_t
#define PDU_DEF asn_DEF_NGAP_NGAP_PDU
#elif defined(PROTOCOL_S1AP)
#include "S1AP_S1AP-PDU.h"
#define PDU_TYPE S1AP_S1AP_PDU_t
#define PDU_DEF asn_DEF_S1AP_S1AP_PDU
#endif

/* The allocations of the program, counted in the place of the C library. */
static unsigned long allocations, octets;
extern void *__libc_malloc(size_t);
extern void *__libc_calloc(size_t, size_t);
extern void *__libc_realloc(void *, size_t);
extern void __libc_free(void *);
void *malloc(size_t size) { allocations++; octets += size; return __libc_malloc(size); }
void *calloc(size_t count, size_t size) { allocations++; octets += count * size; return __libc_calloc(count, size); }
void *realloc(void *pointer, size_t size) { allocations++; octets += size; return __libc_realloc(pointer, size); }
void free(void *pointer) { __libc_free(pointer); }

static double now(void) {
    struct timespec t;
    clock_gettime(CLOCK_MONOTONIC, &t);
    return t.tv_sec * 1e9 + t.tv_nsec;
}

static unsigned char in[65536], out[65536];
static size_t length;
static PDU_TYPE *kept;

static void decode(void) {
    PDU_TYPE *pdu = NULL;
    asn_dec_rval_t read = aper_decode(NULL, &PDU_DEF, (void **)&pdu, in, length, 0, 0);
    if (read.code != RC_OK) { fprintf(stderr, "asn1c does not decode the PDU\n"); exit(1); }
    ASN_STRUCT_FREE(PDU_DEF, pdu);
}

static void encode(void) {
    asn_enc_rval_t written = aper_encode_to_buffer(&PDU_DEF, NULL, kept, out, sizeof out);
    if (written.encoded < 0) { fprintf(stderr, "asn1c does not encode the PDU\n"); exit(1); }
}

int main(int argc, char **argv) {
    if (argc != 4) { fprintf(stderr, "asn1c OPERATION ITERATIONS HEX\n"); return 2; }
    void (*call)(void) = strcmp(argv[1], "encode") == 0 ? encode : decode;
    long iterations = atol(argv[2]);
    for (const char *hex = argv[3]; hex[0] && hex[1] && length < sizeof in; hex += 2) {
        unsigned value;
        sscanf(hex, "%2x", &value);
        in[length++] = (unsigned char)value;
    }
    ogs_core_initialize();
    if (aper_decode(NULL, &PDU_DEF, (void **)&kept, in, length, 0, 0).code != RC_OK) {
        fprintf(stderr, "asn1c does not decode the PDU\n");
        return 1;
    }
    for (long i = 0; i < iterations; i++) call();
    unsigned long before[2] = {allocations, octets};
    call();
    unsigned long count = allocations - before[0], asked = octets - before[1];
    double fastest = 1e300;
    for (int run = 0; run < 3; run++) {
        double started = now();
        for (long i = 0; i < iterations; i++) call();
        double ns = (now() - started) / iterations;
        if (ns < fastest) fastest = ns;
    }
    /* The most memory that the program held: VmHWM of /proc/self/status, in KiB. */
    unsigned long held = 0;
    FILE *status = fopen("/proc/self/status", "r");
    char line[256];
    while (status && fgets(line, sizeof line, status))
        if (sscanf(line, "VmHWM: %lu", &held) == 1) break;
    if (status) fclose(status);
    printf("{\"ns\": %.1f, \"allocations\": %lu, \"bytes\": %lu, \"held\": %lu}\n", fastest, count,
           asked, held);
    return 0;
}
