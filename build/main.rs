mod aper_fix;
mod ngap;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=build/main.rs");
    println!("cargo:rerun-if-changed=build/ngap.rs");
    println!("cargo:rerun-if-changed=build/aper_fix.rs");
    println!("cargo:rerun-if-changed=ngap");

    // docs.rs uses the checked-in generated module and does not need a compiler run.
    if std::env::var("DOCS_RS").is_ok() {
        return Ok(());
    }

    ngap::generate_ngap()?;
    Ok(())
}
