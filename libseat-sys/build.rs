#[cfg(feature = "use_bindgen")]
fn main() {
    use bindgen::Builder;
    println!("cargo:rustc-link-lib=seat");

    println!("cargo:rerun-if-changed=src/wrapper.h");

    let bindings = Builder::default()
        .header("src/wrapper.h")
        .layout_tests(false)
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("./src/bindings.rs")
        .expect("Couldn't write bindings!");
}

#[cfg(not(feature = "use_bindgen"))]
fn main() {
    println!("cargo:rustc-link-lib=seat");
}
