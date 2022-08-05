
fn main() {
    println!("cargo:rustc-link-search=/opt/local/lib/"); 
    println!("cargo:rustc-link-lib=dylib=pcre2-8"); 

    println!("cargo:rerun-if-changed=src/main.rs");
}
