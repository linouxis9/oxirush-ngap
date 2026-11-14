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

//! The tables that the generator writes for NGAP: its registry, and with `inspect` the
//! tables of the trees and of the paths, as oxirush-aper checks those of each protocol.

oxirush_aper::__registry_tests!(oxirush_ngap::Ngap);

#[cfg(feature = "inspect")]
mod inspect {
    oxirush_aper::__inspect_tests!(oxirush_ngap::inspect::__tables());
}
