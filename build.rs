use bindgen::builder;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=parse-lib.h");
    println!("cargo:rerun-if-changed=parse-lib.c");
    println!("cargo:rustc-link-arg=parse-lib.o");
    // println!("cargo:rustc-link-search=native=.");
    // println!("cargo:rustc-link-lib=static=parse-lib");

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let bindings = builder().header("parse-lib.h").allowlist_item("cups_.*").generate()?;
    bindings.write_to_file(Path::new(&out_dir).join("bindings.rs"))?;
    Ok(())
}
