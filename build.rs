// build.rs


fn main() {

    println!("cargo:rustc-link-lib=dylib=test");
    println!("cargo:rustc-link-search=native=./src");
}