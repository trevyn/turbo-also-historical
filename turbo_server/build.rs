fn main() {
 println!("cargo:rustc-env=RUST_VERSION={}", rustc_version::version().unwrap());
}
