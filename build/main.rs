mod aper_fix;
mod containers;
mod inspection;
mod ngap;

fn main() -> anyhow::Result<()> {
    ngap::generate_ngap()
}
