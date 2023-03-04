use std::{env, error::Error, fs::File, io::Write, path::PathBuf};


fn main() -> Result<(), Box<dyn Error>> {
    // Build directory for this crate.
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // Extend the library search path.
    println!("cargo:rustc-link-search={}", out_dir.display());

    // Put `minimal.ld` in the build directory.
    if cfg!(no_copy) {
        File::create(out_dir.join("minimal.ld"))?.write_all(include_bytes!("minimal.no_copy.ld"))?;
    } else {
        File::create(out_dir.join("minimal.ld"))?.write_all(include_bytes!("minimal.ld"))?;
    }

    Ok(())
}
