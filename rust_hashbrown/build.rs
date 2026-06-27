use std::env;



fn main() {

    let current_dir = env::current_dir().unwrap();

    let lib_path = current_dir.join("../f_count/target/release").canonicalize().unwrap();

    let lib_dir = lib_path.to_str().unwrap();



    println!("cargo:rustc-link-search=native={}", lib_dir);

    println!("cargo:rustc-link-lib=dylib=f_count");

    

    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_dir);

}
