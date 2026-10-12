mod aper_fix;
mod asn1;
mod containers;
mod inspection;
mod ngap;
mod registry;

fn main() -> anyhow::Result<()> {
    ngap::generate_ngap()
}
