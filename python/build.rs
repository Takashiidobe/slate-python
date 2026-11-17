fn main() {
    println!("cargo:rerun-if-changed=src/slate_long_double.c");
    cc::Build::new().file("src/slate_long_double.c").compile("slate_long_double");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("linux") {
        println!("cargo:rustc-link-arg-bins=-Wl,-z,nostart-stop-gc");
    }
}
