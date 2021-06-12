fn main() {
    #[cfg(feature = "custom_logger")]
    cc::Build::new()
        .file("./log_handler/log_handler.c")
        .flag("-Wno-unused-parameter")
        .compile("libpreformatedlog");
}
