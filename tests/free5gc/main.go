// OxiRush
// Copyright 2026 Valentin D'Emmanuele
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this
// file except in compliance with the License. You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0. Unless required by applicable law or agreed
// to in writing, software distributed under the License is distributed on an "AS IS"
// BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the
// License for the specific language governing permissions and limitations under the
// License.

// What the NGAP library of free5GC reads of each PDU of a corpus, for the test free5gc of
// the crate:
//
//	free5gc IDS CORPUS
//
// IDS is the table that ids.py writes of the library: the type of a message, a field and
// the id of its IE. CORPUS has a PDU a row, by tabs, its octets in the last column. For each
// row, a line of JSON: its octets, whether message.Parse reads it, or its error; the type
// of the PDU (0 initiating, 1 successful, 2 unsuccessful), the procedure code and the type
// of the message; each IE that the message has, in the order of the fields of its type,
// by its id, with the octets of its value as the IE's Write writes them; and whether the
// message that MarshalBinary writes is the PDU. A panic of the library is caught, and is
// its error.
package main

import (
	"bufio"
	"bytes"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"os"
	"reflect"
	"strconv"
	"strings"

	"github.com/free5gc/ngap/aper"
	"github.com/free5gc/ngap/message"
)

type ie struct {
	ID    int64  `json:"id"`
	Field string `json:"field"`
	Value string `json:"value,omitempty"`
	Error string `json:"error,omitempty"`
}

type reading struct {
	Octets    string `json:"octets"`
	Accepted  bool   `json:"accepted"`
	Error     string `json:"error,omitempty"`
	PDU       *int64 `json:"pdu,omitempty"`
	Procedure *int64 `json:"procedure,omitempty"`
	Type      string `json:"type,omitempty"`
	IEs       []ie   `json:"ies,omitempty"`
	Same      bool   `json:"same"`
	Written   string `json:"written,omitempty"`
}

// read is what the library reads of octets.
func read(octets []byte, ids map[string]map[string]int64) (r reading) {
	defer func() {
		if p := recover(); p != nil {
			r = reading{Error: fmt.Sprintf("panic: %v", p)}
		}
	}()
	m, err := message.Parse(append([]byte(nil), octets...))
	if err != nil || m == nil || reflect.ValueOf(m).IsNil() {
		if err == nil {
			err = fmt.Errorf("no message")
		}
		return reading{Error: firstLine(err.Error())}
	}
	r.Accepted = true
	pdu, procedure := m.MessageType(), m.ProcedureCode()
	r.PDU, r.Procedure = &pdu, &procedure
	kind := reflect.TypeOf(m).Elem()
	r.Type = kind.Name()
	value := reflect.ValueOf(m).Elem()
	for i := 0; i < kind.NumField(); i++ {
		field := value.Field(i)
		if field.Kind() != reflect.Ptr || field.IsNil() {
			continue
		}
		entry := ie{Field: kind.Field(i).Name, ID: -1}
		if id, ok := ids[r.Type][entry.Field]; ok {
			entry.ID = id
		}
		pd := aper.NewPerBitData(nil)
		write := field.MethodByName("Write")
		if octets, ok := field.Interface().(*aper.OctetString); ok {
			// An OCTET STRING, as ie.WriteOctetStringIE writes it in its open type
			if err := pd.WriteOctetString(*octets, false, nil, nil); err != nil {
				entry.Error = firstLine(err.Error())
			} else {
				entry.Value = hex.EncodeToString(pd.Bytes())
			}
		} else if !write.IsValid() {
			entry.Error = "no Write"
		} else if out := write.Call([]reflect.Value{reflect.ValueOf(pd)}); !out[0].IsNil() {
			entry.Error = firstLine(out[0].Interface().(error).Error())
		} else {
			entry.Value = hex.EncodeToString(pd.Bytes())
		}
		r.IEs = append(r.IEs, entry)
	}
	if written, err := m.MarshalBinary(); err == nil {
		r.Same = bytes.Equal(written, octets)
		if !r.Same {
			r.Written = hex.EncodeToString(written)
		}
	}
	return r
}

func firstLine(text string) string {
	line, _, _ := strings.Cut(text, "\n")
	return line
}

func rows(file string) *bufio.Scanner {
	f, err := os.Open(file)
	if err != nil {
		panic(err)
	}
	scanner := bufio.NewScanner(f)
	scanner.Buffer(make([]byte, 1<<20), 1<<24)
	return scanner
}

func main() {
	if len(os.Args) != 3 {
		fmt.Fprintln(os.Stderr, "free5gc IDS CORPUS")
		os.Exit(2)
	}
	ids := map[string]map[string]int64{}
	for scanner := rows(os.Args[1]); scanner.Scan(); {
		f := strings.Split(scanner.Text(), "\t")
		if len(f) != 3 {
			continue
		}
		id, err := strconv.ParseInt(f[2], 10, 64)
		if err != nil {
			panic(err)
		}
		if ids[f[0]] == nil {
			ids[f[0]] = map[string]int64{}
		}
		ids[f[0]][f[1]] = id
	}
	out := bufio.NewWriter(os.Stdout)
	defer out.Flush()
	encoder := json.NewEncoder(out)
	for scanner := rows(os.Args[2]); scanner.Scan(); {
		line := scanner.Text()
		if line == "" || strings.HasPrefix(line, "#") {
			continue
		}
		f := strings.Split(line, "\t")
		octets, err := hex.DecodeString(f[len(f)-1])
		if err != nil {
			panic(err)
		}
		r := read(octets, ids)
		r.Octets = f[len(f)-1]
		if err := encoder.Encode(r); err != nil {
			panic(err)
		}
	}
}
