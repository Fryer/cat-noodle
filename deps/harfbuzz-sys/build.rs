#[cfg(feature = "build-native-harfbuzz")]
extern crate cc;
#[cfg(feature = "build-native-harfbuzz")]
extern crate pkg_config;

#[cfg(feature = "build-native-harfbuzz")]
fn main() {
    use std::env;
    use std::path::PathBuf;

    let target = env::var("TARGET").unwrap();

    println!("cargo:rerun-if-env-changed=HARFBUZZ_SYS_NO_PKG_CONFIG");
    if target.contains("wasm32") || env::var_os("HARFBUZZ_SYS_NO_PKG_CONFIG").is_none() {
        if pkg_config::probe_library("harfbuzz").is_ok() {
            return;
        }
    }

    let mut cfg = cc::Build::new();
    cfg.cpp(true)
        .warnings(false)
        .file("harfbuzz/src/harfbuzz.cc")
        .include("freetype/include");

    if !target.contains("windows") {
        cfg.define("HAVE_PTHREAD", "1");
    }

    if target.contains("apple") {
        cfg.define("HAVE_CORETEXT", "1");
    }

    cfg.define("HAVE_FREETYPE", "1");

    cfg.compile("embedded_harfbuzz");

    let out_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());

    println!(
        "cargo:include={}",
        out_dir.join("harfbuzz").join("src").display()
    );
}

#[cfg(not(feature = "build-native-harfbuzz"))]
fn main() {}
