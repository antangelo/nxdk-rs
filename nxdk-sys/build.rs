// SPDX-License-Identifier: MIT

extern crate bindgen;

fn link_lib(lib: &str) {
    println!("cargo:rustc-link-lib={}", lib);
}

fn gen_bindings(nxdk_dir: &str, lib_path: &str, header: &str) {
    let bindings = bindgen::builder()
        .header(format!("{}/lib/{}/{}.h", nxdk_dir, lib_path, header))
        .clang_arg(format!("-I{}/lib", nxdk_dir))
        .clang_arg(format!("-I{}/lib/xboxrt/libc_extensions", nxdk_dir))
        .clang_arg(format!("-I{}/lib/pdclib/include", nxdk_dir))
        .clang_arg(format!("-I{}/lib/pdclib/platform/xbox/include", nxdk_dir))
        .clang_arg(format!("-I{}/lib/winapi", nxdk_dir))
        .clang_arg(format!("-I{}/lib/xboxrt/vcruntime", nxdk_dir))
        .clang_arg("-D__STDC__=1")
        .clang_arg("-DNXDK")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .use_core()
        .ctypes_prefix("libc")
        .generate()
        .expect("Unable to generate bindings");

    println!("cargo:rerun-if-changed={}/lib/{}/{}/h", nxdk_dir, lib_path, header);
    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join(format!("bindings_{}.rs", header))).expect("Unable to write bindings");
}

fn main() {
    let nxdk_dir;
    match std::env::var("NXDK_DIR") {
        Ok(v) => {
            nxdk_dir = v;
        },
        Err(e) => {
            panic!("Error reading NXDK_DIR environment variable: {:?}", e);
        }
    }

    println!("cargo:rustc-link-search=native={}/lib", nxdk_dir);
    println!("cargo:rustc-link-search=native={}/lib/xboxkrnl", nxdk_dir);

    link_lib("libpdclib");
    link_lib("libwinapi");
    link_lib("libnxdk");
    link_lib("libnxdk_hal");
    link_lib("libnxdk_automount_d");
    link_lib("libzlib");
    link_lib("libxboxrt");
    link_lib("libSDL2");
    link_lib("libSDL2_image");
    link_lib("libSDL_ttf");
    link_lib("libfreetype");
    link_lib("libjpeg");
    link_lib("libpng");
    link_lib("libpbkit");
    link_lib("libxboxkrnl");
    link_lib("winmm");

    gen_bindings(&nxdk_dir, "hal", "audio");
    gen_bindings(&nxdk_dir, "hal", "debug");
    gen_bindings(&nxdk_dir, "hal", "fileio");
    gen_bindings(&nxdk_dir, "hal", "led");
    gen_bindings(&nxdk_dir, "hal", "video");
    gen_bindings(&nxdk_dir, "hal", "xbox");
    
    gen_bindings(&nxdk_dir, "pbkit", "pbkit");

    gen_bindings(&nxdk_dir, "xboxkrnl", "xboxkrnl");

    gen_bindings(&nxdk_dir, "pdclib/include", "stdlib");
    gen_bindings(&nxdk_dir, "pdclib/include", "string");
    gen_bindings(&nxdk_dir, "pdclib/include", "stdio");
    gen_bindings(&nxdk_dir, "pdclib/include", "time");

    gen_bindings(&nxdk_dir, "winapi", "windows");
}
