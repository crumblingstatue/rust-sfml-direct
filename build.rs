use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rustc-link-lib=sfml-system");
    println!("cargo:rustc-link-lib=sfml-audio");
    println!("cargo:rustc-link-lib=sfml-window");
    println!("cargo:rustc-link-lib=sfml-graphics");

    let bindings = bindgen::Builder::default()
        .clang_args(&["-x", "c++"])
        .header("wrapper.h")
        .whitelist_type("sf::Clock")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
