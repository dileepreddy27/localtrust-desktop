fn main() {
    cc::Build::new().cpp(true).file("native/gguf.cpp").compile("gguf_header");
    println!("cargo:rerun-if-changed=native/gguf.cpp");
}
