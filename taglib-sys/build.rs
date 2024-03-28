#[cfg(feature = "pkg-config")]
extern crate pkg_config;

extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    if !build_pkgconfig() {
        build_fixed();
    }
}

fn build_fixed() {
    let headers_path = PathBuf::from("/opt/usr/local/include/taglib")
        // Canonicalize the path as `rustc-link-search` requires an absolute
        // path.
        .canonicalize()
        .expect("cannot canonicalize path");
    let headers_path = headers_path.join("tag_c.h");
    let headers_path_str = headers_path.to_str().expect("Path is not a valid string");

    let libdir_path = PathBuf::from("/opt/usr/local/lib")
        // Canonicalize the path as `rustc-link-search` requires an absolute
        // path.
        .canonicalize()
        .expect("cannot canonicalize path");
    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search={}", libdir_path.to_str().unwrap());

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=tag_c");
    println!("cargo:rustc-link-lib=tag");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(headers_path_str)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    // let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    // bindings
    //     .write_to_file(out_path.join("bindings.rs"))
    //     .expect("Couldn't write bindings!");
    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap())
        .join("bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}

#[cfg(not(feature = "pkg-config"))]
fn build_pkgconfig() -> bool {
    false
}

#[cfg(feature = "pkg-config")]
fn build_pkgconfig() -> bool {
    let library = pkg_config::probe_library("taglib_c").expect("probe error");

    let headers_path_str = library.include_paths.iter()
        .map(|p| p.join("tag_c.h"))
        .map(|e| e.canonicalize())
        .filter(|e| e.is_ok())
        .map(|e| e.unwrap())
        .filter(|e| e.exists())
        .nth(0).expect("Not found header file")
        .to_str().expect("header file name contains illegal character.")
        .to_string();

    let libdir_path = library.link_paths.iter()
        .map(|e| e.canonicalize())
        .filter(|e| e.is_ok())
        .map(|e| e.unwrap())
        .filter(|e| e.exists())
        .nth(0).expect("Not found library path")
        .to_str().expect("library path name contains illegal character.")
        .to_string();
    println!("cargo:rustc-link-search={}", &libdir_path);

    println!("cargo:rustc-link-lib=tag_c");
    println!("cargo:rustc-link-lib=tag");

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(&headers_path_str)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap())
        .join("bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");

    true
}