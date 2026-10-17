// OxiRush
// Copyright 2025-2026 Valentin D'Emmanuele
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this
// file except in compliance with the License. You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0. Unless required by applicable law or agreed
// to in writing, software distributed under the License is distributed on an "AS IS"
// BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the
// License for the specific language governing permissions and limitations under the
// License.

// One operation of the ngap package of free5GC on one PDU, as compare.py runs it:
//
//	free5gc OPERATION ITERATIONS HEX
//
// OPERATION is `decode`, the PDU decoded with every IE of its type, as the package
// decodes, or `encode`, that PDU written again. It runs the operation ITERATIONS times
// as a warm-up, then three times ITERATIONS times, and prints {"ns": N, "allocations": A,
// "bytes": B, "held": H, "same": S}: the time of a call in the fastest of the three, the
// allocations and the octets that the runtime counts for a call, the most memory that the
// program held, in KiB, and whether the PDU written is the one read.
package main

import (
	"bytes"
	"encoding/hex"
	"fmt"
	"os"
	"runtime"
	"strconv"
	"strings"
	"time"

	"github.com/free5gc/ngap/message"
)

func main() {
	if len(os.Args) != 4 {
		fmt.Fprintln(os.Stderr, "free5gc OPERATION ITERATIONS HEX")
		os.Exit(2)
	}
	iterations, _ := strconv.Atoi(os.Args[2])
	octets, _ := hex.DecodeString(os.Args[3])
	pdu, err := message.Parse(octets)
	if err != nil {
		fmt.Fprintln(os.Stderr, "free5GC does not decode the PDU:", err)
		os.Exit(1)
	}
	written, err := pdu.MarshalBinary()
	if err != nil {
		fmt.Fprintln(os.Stderr, "free5GC does not encode the PDU:", err)
		os.Exit(1)
	}
	call := func() {
		if _, err := message.Parse(octets); err != nil {
			panic(err)
		}
	}
	if os.Args[1] == "encode" {
		call = func() {
			if _, err := pdu.MarshalBinary(); err != nil {
				panic(err)
			}
		}
	}
	for i := 0; i < iterations; i++ {
		call()
	}
	var before, after runtime.MemStats
	runtime.ReadMemStats(&before)
	for i := 0; i < iterations; i++ {
		call()
	}
	runtime.ReadMemStats(&after)
	count := (after.Mallocs - before.Mallocs) / uint64(iterations)
	asked := (after.TotalAlloc - before.TotalAlloc) / uint64(iterations)
	fastest := 1e300
	for run := 0; run < 3; run++ {
		started := time.Now()
		for i := 0; i < iterations; i++ {
			call()
		}
		ns := float64(time.Since(started).Nanoseconds()) / float64(iterations)
		if ns < fastest {
			fastest = ns
		}
	}
	// The most memory that the program held: VmHWM of /proc/self/status, in KiB.
	held := 0
	if status, err := os.ReadFile("/proc/self/status"); err == nil {
		for _, line := range strings.Split(string(status), "\n") {
			if strings.HasPrefix(line, "VmHWM:") {
				held, _ = strconv.Atoi(strings.Fields(line)[1])
			}
		}
	}
	fmt.Printf("{\"ns\": %.1f, \"allocations\": %d, \"bytes\": %d, \"held\": %d, \"same\": %t}\n",
		fastest, count, asked, held, bytes.Equal(written, octets))
}
