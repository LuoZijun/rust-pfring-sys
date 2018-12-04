
fn main() {
    // println!("cargo:rustc-link-search=dylib=/usr/local/lib");
    // println!("cargo:rustc-link-search=static=/usr/local/lib");
    println!("cargo:rustc-flags=-l static=pfring -L /usr/local/lib");
    println!("cargo:rustc-flags=-l static=pcap -L /usr/local/lib");
}