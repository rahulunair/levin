use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-rerun-if-changed=./lib/levenshtein.c");
    println!("cargo:rustc-link-search=native=.");
    println!("cargo:rustc-link-lib=static=levenshtein");

    cc::Build::new()
        .file("lib/levenshtein.c")
        .out_dir(".")
        .compile("levenshtein");

    let bindings = bindgen::Builder::default()
        .header("lib/levenshtein.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("./src/");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
