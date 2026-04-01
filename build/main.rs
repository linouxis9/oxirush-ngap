mod ngap;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // On docs.rs, skip code generation — the pre-generated src/ngap.rs is already in the package.
    if std::env::var("DOCS_RS").is_ok() {
        return Ok(());
    }

    ngap::generate_ngap()?;

    Ok(())
}
