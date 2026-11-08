//! The generator of the bindings of the crate: the generator of oxirush-aper, which
//! oxirush-ngap and oxirush-s1ap share, with the names of NGAP. It runs rasn-compiler on
//! the ASN.1 of the specification, and writes the bindings, the registry, the inspection
//! and the oracle of the tests.

use generator::protocol::{Protocol, generate};

/// The IEs whose octets ASN.1 leaves open, each with the type of this module that TS
/// 38.413 gives them for a handover between NG-RAN nodes: clauses 9.3.1.20, 9.3.1.21
/// and 9.3.1.186. For another target system the octets are those of its own
/// specification.
const CARRIED: &[(&str, &str)] = &[
    (
        "SourceToTarget-TransparentContainer",
        "SourceNGRANNode-ToTargetNGRANNode-TransparentContainer",
    ),
    (
        "TargetToSource-TransparentContainer",
        "TargetNGRANNode-ToSourceNGRANNode-TransparentContainer",
    ),
    (
        "TargettoSource-Failure-TransparentContainer",
        "TargetNGRANNode-ToSourceNGRANNode-FailureTransparentContainer",
    ),
];

/// The IEs whose octets carry a type of S1AP at a handover to EPS, each with that type:
/// TS 38.413 clauses 9.3.1.20 and 9.3.1.21 give them the containers of TS 36.413. The
/// tables of oxirush-s1ap read them, once a program registers them.
const FOREIGN: &[(&str, &str, &str)] = &[
    (
        "SourceToTarget-TransparentContainer",
        "S1AP",
        "SourceeNB-ToTargeteNB-TransparentContainer",
    ),
    (
        "TargetToSource-TransparentContainer",
        "S1AP",
        "TargeteNB-ToSourceeNB-TransparentContainer",
    ),
];

fn main() -> anyhow::Result<()> {
    generate(&Protocol {
        name: "NGAP",
        modules: "ngap",
        carried: CARRIED,
        foreign: FOREIGN,
        keys: &["SecurityKey"],
        plmn: "PLMNIdentity",
        // The bindings that rasn-compiler resolves need nothing else.
        correct: Ok,
    })
}
