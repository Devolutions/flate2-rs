extern crate cc;

use std::env;
use std::path::Path;

extern crate conan;
use conan::*;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let conan_profile = format!("{}-{}", target_os, target_arch);

    let command = InstallCommandBuilder::new()
        .with_profile(&conan_profile)
        .build_policy(BuildPolicy::Never)
        .recipe_path(Path::new("conanfile.txt"))
        .build();

    if let Some(build_info) = command.generate() {
        println!("using conan build info");
        build_info.cargo_emit();
        return;
    }
    
    let target = env::var("TARGET").unwrap();
    if target.starts_with("wasm32-") && !target.ends_with("-emscripten") {
        return;
    }
    let mut build = cc::Build::new();
    build
        .file("miniz.c")
        .define("MINIZ_NO_STDIO", None)
        .define("MINIZ_NO_ARCHIVE_APIS", None)
        .define("MINIZ_NO_ARCHIVE_WRITING_APIS", None)
        .define("MINIZ_NO_TIME", None)
        .define("MINIZ_NO_ZLIB_COMPATIBLE_NAMES", None)
        .warnings(false);

    if !target.contains("darwin") && !target.contains("windows") {
        build.flag("-fvisibility=hidden");
    }

    build.compile("miniz");
    println!("cargo:root={}", env::var("OUT_DIR").unwrap());
}
