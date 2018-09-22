extern crate failure;

use std::{env, fs::File, io::Write, path::PathBuf};

fn main() -> Result<(), failure::Error> {
    // Put the linker script somewhere the linker can find it
    let out = PathBuf::from(env::var("OUT_DIR")?);

    File::create(out.join("stlog.x"))?.write_all(include_bytes!("stlog.x"))?;

    println!("cargo:rustc-link-search={}", out.display());

    Ok(())
}
