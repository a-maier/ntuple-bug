fn main() {
    for flag in ntuple::ROOT_LINKER_FLAGS {
        println!("cargo:rustc-link-arg={flag}");
    }
   std::env::var("CARGO_MANIFEST_DIR");
}
