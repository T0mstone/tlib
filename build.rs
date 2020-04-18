fn main() {
    if let Ok(s) = std::env::var("PROFILE") {
        // values: debug, release
        println!("cargo:rustc-cfg=profile=\"{}\"", s);
        println!("cargo:rustc-env=profile=\"{}\"", s);
    }
}
