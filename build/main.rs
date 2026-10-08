mod aper_fix;
mod containers;
mod inspection;
mod ngap;
mod registry;

fn main() -> anyhow::Result<()> {
    ngap::generate_ngap()
}
