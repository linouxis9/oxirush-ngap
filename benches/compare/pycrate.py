# OxiRush
# Copyright 2025-2026 Valentin D'Emmanuele
#
# Licensed under the Apache License, Version 2.0 (the "License"); you may not use this
# file except in compliance with the License. You may obtain a copy of the License at
# http://www.apache.org/licenses/LICENSE-2.0. Unless required by applicable law or agreed
# to in writing, software distributed under the License is distributed on an "AS IS"
# BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the
# License for the specific language governing permissions and limitations under the
# License.

"""One operation of pycrate on one PDU, as compare.py runs it:

    python3 pycrate.py PROTOCOL OPERATION ITERATIONS HEX

PROTOCOL is `ngap` or `s1ap`, and OPERATION `decode`, the PDU decoded with every IE of
its type, or `encode`, that PDU written again. It runs the operation ITERATIONS times
as a warm-up, then three times ITERATIONS times, and prints {"ns": N, "held": H}: the
time of a call in the fastest of the three, and the most memory that the program held,
in KiB. Python allocates in its own pools, which no number here counts. This file is the same in oxirush-ngap and oxirush-s1ap.
"""

import json
import sys
import time


def main():
    protocol, operation, iterations, octets = sys.argv[1:]
    iterations, octets = int(iterations), bytes.fromhex(octets)
    if protocol == "ngap":
        from pycrate_asn1dir import NGAP
        pdu = NGAP.NGAP_PDU_Descriptions.NGAP_PDU
    else:
        from pycrate_asn1dir import S1AP
        pdu = S1AP.S1AP_PDU_Descriptions.S1AP_PDU
    pdu.from_aper(octets)
    call = pdu.to_aper if operation == "encode" else (lambda: pdu.from_aper(octets))
    for _ in range(iterations):
        call()
    fastest = float("inf")
    for _ in range(3):
        started = time.perf_counter()
        for _ in range(iterations):
            call()
        fastest = min(fastest, (time.perf_counter() - started) / iterations * 1e9)
    # The most memory that the program held: VmHWM of /proc/self/status, in KiB.
    held = 0
    with open("/proc/self/status") as status:
        for line in status:
            if line.startswith("VmHWM:"):
                held = int(line.split()[1])
    print(json.dumps({"ns": round(fastest, 1), "held": held}))


main()
