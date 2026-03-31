mod ngap;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    ngap::generate_ngap()?;

    Ok(())
}
