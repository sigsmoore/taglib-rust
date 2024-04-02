#[cfg(feature = "pkg-config")]
extern crate pkg_config;
extern crate core;

use std::collections::HashSet;
use std::env;
use std::path::PathBuf;

//是否静态编译，默认否
// TAGLIB_STATIC=1
// 多个用半角冒号`:`分隔，Windows下用半角分号`;`分隔，示例
// TAGLIB_LIB_DIRS=/others/lib:/opt/usr/local/lib
// 多个用半角冒号`:`分隔，标准的`tag_c`和`tag`无需指定，示例
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
    get_env_hashset_string("TAGLIB_EXTRA_LIBS", ':')
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
    get_env_hashset_string("TAGLIB_LIB_DIRS", sep).into_iter()
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
    match &get_env_string("TAGLIB_STATIC") {
        None => "dylib",
        Some(v) => if v.eq("0") { "dylib" } else { "static" }
    }
}

fn get_env_string(env_key: &str) -> Option<String> {
    println!("cargo:rerun-if-env-changed={}", env_key);
    env::var("OPENCC_STATIC").ok()
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
