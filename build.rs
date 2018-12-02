extern crate bindgen;

use std::fs;
use std::env;
use std::path;
use std::process;
use std::io::{ Write, };


fn main() {
//     let current_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
//     let out_path = path::PathBuf::from(env::var("OUT_DIR").unwrap());

//     let include_dir = path::Path::new(&current_dir).join("include");

    
//     let pfring_header = format!("
// #include <stdbool.h>
// #include \"{}/pfring_ft.h\"
// #include \"{}/pfring_mod_sysdig.h\"
// #include \"{}/pfring_zc.h\"
// #include \"{}/pfring_mod_stack.h\"
// #include \"{}/pfring_utils.h\"
// #include \"{}/pfring.h\"
// #include \"{}/linux/pfring.h\"

// ", &include_dir.clone().as_path().to_string_lossy(),
//     &include_dir.clone().as_path().to_string_lossy(),
//     &include_dir.clone().as_path().to_string_lossy(),
//     &include_dir.clone().as_path().to_string_lossy(),
//     &include_dir.clone().as_path().to_string_lossy(),
//     &include_dir.clone().as_path().to_string_lossy(),
//     &include_dir.clone().as_path().to_string_lossy());

//     let mut file = fs::OpenOptions::new().write(true).create(true).append(false)
//         .open(&out_path.join("pfring.h").as_path())
//         .unwrap();
//     file.write_all(&pfring_header.as_bytes()).unwrap();

//     bindgen::Builder::default()
//         .header(out_path.join("pfring.h").as_path().to_string_lossy())
//         .impl_debug(true)
//         .impl_partialeq(true)
//         .derive_copy(true)
//         .derive_debug(true)
//         .derive_default(true)
//         .derive_hash(true)
//         .derive_partialeq(true)
//         .derive_eq(true)
//         .prepend_enum_name(true)
//         .default_enum_style(bindgen::EnumVariation::Rust)
//         .generate()
//         .expect("Unable to generate bindings")
//         .write_to_file(out_path.join("pfring.rs").as_path())
//         .expect("Couldn't write bindings!");;

//     // println!("cargo:rustc-link-lib=c++");
//     // println!("cargo:rustc-link-search=static={}", &openh264_source_dir.display());
//     // println!("cargo:rustc-flags=-l dylib=stdc++");
//     // println!("cargo:rustc-flags=-l static=openh264 -L {}", &openh264_source_dir.display());
//     // println!("cargo:rerun-if-changed={}", &openh264_source_dir.display());

    println!("cargo:rustc-link-search=static=/usr/local/lib");
    println!("cargo:rustc-flags=-l static=pfring -L /usr/local/lib");
    println!("cargo:rustc-flags=-l static=pcap -L /usr/local/lib");

}