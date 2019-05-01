extern crate phf_codegen;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn main() {
    // The build script doesn't depend on any files
    // Don't re-run it whenever a file changes
    // It will be automatically re-run when features change
    println!("cargo:rerun-if-changed=build.rs");
    if let Ok(profile) = env::var("PROFILE") {
        println!("cargo:rustc-cfg=build={:?}", profile);
    }

    let feature_prefix = "CARGO_FEATURE_";
    let out_dir = env::var("OUT_DIR").unwrap();

    let mut crates = Vec::new();
    for (key, val) in env::vars() {
        if val == "1" && key.starts_with(feature_prefix) {
            let krate = key[feature_prefix.len()..].to_lowercase();
            match krate.as_ref() {
                "default" | "unix" | "redox" | "redox_generic" | "fuchsia" | "generic"
                | "nightly" | "test_unimplemented" => continue,
                _ => {}
            }
            crates.push(krate);
        }
    }
    crates.sort();

    let mut cf = File::create(Path::new(&out_dir).join("uutils_crates.rs")).unwrap();
    let mut mf = File::create(Path::new(&out_dir).join("uutils_map.rs")).unwrap();

    mf.write_all(
        b"
    type UtilityMap = phf::Map<&'static str, fn(Vec<String>) -> i32>;

    fn util_map() -> &'static UtilityMap { &UTIL_MAP }
    static UTIL_MAP: UtilityMap = "
    ).unwrap();

    let mut map = phf_codegen::Map::new();
    for krate in &crates {
        cf.write_all(format!("extern crate uu_{krate};\n", krate = krate).as_bytes())
            .unwrap();

        if krate == "hashsum" {
            map.entry("md5sum", "uu_hashsum::uumain as fn(Vec<String>) -> i32");
            map.entry("sha1sum", "uu_hashsum::uumain");
            map.entry("sha224sum", "uu_hashsum::uumain");
            map.entry("sha256sum", "uu_hashsum::uumain");
            map.entry("sha384sum", "uu_hashsum::uumain");
            map.entry("sha512sum", "uu_hashsum::uumain");
            map.entry("sha3sum", "uu_hashsum::uumain");
            map.entry("sha3-224sum", "uu_hashsum::uumain");
            map.entry("sha3-256sum", "uu_hashsum::uumain");
            map.entry("sha3-384sum", "uu_hashsum::uumain");
            map.entry("sha3-512sum", "uu_hashsum::uumain");
            map.entry("shake128sum", "uu_hashsum::uumain");
            map.entry("shake256sum", "uu_hashsum::uumain");
        }
        map.entry(&krate, &format!("uu_{krate}::uumain as fn(Vec<String>) -> i32", krate=krate));
    }
    
    map.build(&mut mf).unwrap();
    mf.write_all(b";\n").unwrap();

    cf.flush().unwrap();
    mf.flush().unwrap();
}
