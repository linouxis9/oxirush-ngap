mod aper_fix;
mod inspection;
mod ngap;

fn main() -> anyhow::Result<()> {
    ngap::generate_ngap()
}
