#[cfg(feature = "use_bindgen")]
fn main() {
    use bindgen::Builder;
    println!("cargo:rustc-link-lib=seat");

    println!("cargo:rerun-if-changed=src/wrapper.h");

    let bindings = Builder::default()
        .header("src/wrapper.h")
        .layout_tests(false)
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .allowlist_recursively(false)
        .allowlist_type("libseat.*")
        .allowlist_function("libseat.*")
        .allowlist_var("libseat.*")
        .blocklist_item("libseat_set_log_handler")
        .blocklist_item("libseat_log_func")
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
