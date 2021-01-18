// build.rs
extern crate bindgen;

use std::{
    env,
    path::PathBuf,
};
//use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    println!("cargo:rerun-if-changed=wrapper.hpp");

    eprintln!("Building PTex");
    /*let ptex = vcpkg::Config::new()
        .emit_includes(true)
        .find_package("ptex")
        .unwrap();

    println!("cargo:warning={:?}", ptex.include_paths);*/

    let ptex_path = cmake::build("ptex");

    let mut ptex_include_path = ptex_path.clone();
    ptex_include_path.push("include");

    let mut ptex_lib_path = ptex_path.clone();
    ptex_lib_path.push("lib");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    println!("cargo:warning={:?}", ptex_include_path);

    // Emit linker searchpath
    println!("cargo:rustc-link-search={:?}", ptex_lib_path);
    // Link to lib3delight
    println!("cargo:rustc-link-lib=dylib=ptex");

    let bindings = bindgen::Builder::default()
        .header("wrapper.hpp")
        .derive_debug(true)
        .whitelist_type("Ptex.*")
        .rustified_enum("Ptex.*")
        .disable_name_namespacing()
        .default_alias_style(bindgen::AliasVariation::NewTypeDeref)
        .clang_arg("-std=c++11")
        .clang_arg("-stdlib=libc++")
        .clang_arg("-isysroot/Library/Developer/CommandLineTools/SDKs/MacOSX10.15.sdk")
        .clang_arg(format!("-I{}", ptex_include_path.display()))
        .generate()
        .expect("Unable to generate ptex bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Could not write bindings.");

    Ok(())
}
