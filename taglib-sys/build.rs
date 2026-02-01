#[cfg(feature = "pkg-config")]
extern crate pkg_config;
extern crate core;

use std::collections::HashSet;
use std::env;
use std::path::PathBuf;

const KEY_TAGLIB_STATIC: &'static str = "TAGLIB_STATIC";
const KEY_TAGLIB_DIRS: &'static str = "TAGLIB_LIB_DIRS";
const KEY_TAGLIB_EXTRA_LIBS: &'static str = "TAGLIB_EXTRA_LIBS";

// if not empty and not zero, build as static link, default is dynamic link (dll/so/dylib), example:
// TAGLIB_STATIC=1
// multiple dir separated by char `:` in Unix/Linux/Mac, ';' in Windows, example:
// TAGLIB_LIB_DIRS=/others/lib:/opt/usr/local/lib
// multiple name separated by char `:`, example:
// TAGLIB_EXTRA_LIBS=zlib
fn main() {
    if !build_pkgconfig() {
        build_env();
    }
}

fn build_env() {
    let sep = get_sep();

    let lib_dirs = get_lib_dirs(sep);
    for dir in &lib_dirs {
        if !dir.exists() {
            panic!("library directory does not exist: {}", dir.to_string_lossy());
        }
        println!("cargo:rustc-link-search=native={}", dir.to_string_lossy());
    }

    let kind = get_link_mode();
    let extra_libs = get_extra_libs();
    for lib in &extra_libs {
        println!("cargo:rustc-link-lib={}={}", kind, lib);
    }

    println!("cargo:rustc-link-lib={}={}", kind, "tag_c");
    println!("cargo:rustc-link-lib={}={}", kind, "tag");
}

fn get_extra_libs() -> HashSet<String> {
    get_env_hashset_string(KEY_TAGLIB_EXTRA_LIBS, ':')
}

fn get_sep() -> char {
    match env::var("TARGET") {
        Ok(ref t) => {
            if t.to_lowercase().contains("windows") {
                ';'
            } else {
                ':'
            }
        }
        _ => ':'
    }
}

fn get_lib_dirs(sep: char) -> Vec<PathBuf> {
    get_env_hashset_string(KEY_TAGLIB_DIRS, sep).into_iter()
        .map(PathBuf::from).collect::<Vec<PathBuf>>()
}

fn get_env_hashset_string(env_key: &str, sep: char) -> HashSet<String> {
    println!("cargo:rerun-if-env-changed={}", env_key);
    env::var(env_key)
        .map_or_else(|_| HashSet::new(),
                     |v| v.split(sep)
                         .map(|e| e.trim().to_owned()).collect::<HashSet<String>>())
}

fn get_link_mode() -> &'static str {
    match &get_env_string(KEY_TAGLIB_STATIC) {
        None => "dylib",
        Some(v) => if v.eq("0") { "dylib" } else { "static" }
    }
}

fn get_env_string(env_key: &str) -> Option<String> {
    println!("cargo:rerun-if-env-changed={}", env_key);
    env::var(KEY_TAGLIB_STATIC).ok()
}

#[cfg(not(feature = "pkg-config"))]
fn build_pkgconfig() -> bool {
    false
}

#[cfg(feature = "pkg-config")]
fn build_pkgconfig() -> bool {
    if pkg_config::find_library("taglib_c").is_err() {
        panic!("Could not find taglib_c via pkgconfig");
    }
    true
}
