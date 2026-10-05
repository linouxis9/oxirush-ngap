mod aper_fix;
mod ngap;

fn main() -> anyhow::Result<()> {
    ngap::generate_ngap()
}
