//! The generator of the bindings of the crate: it runs rasn-compiler on the ASN.1 of
//! the specification, and writes the bindings, the registry and the oracle of the
//! tests with the generator of oxirush-aper, which oxirush-ngap and oxirush-s1ap share.

mod ngap;

fn main() -> anyhow::Result<()> {
    ngap::generate_ngap()
}
