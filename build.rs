use std::path::PathBuf;

fn main() {
    // 1. 编译 C 代码
    cc::Build::new()
    .file("csrc/calc.c")
    .include("csrc")
    .compile("calc");

    // 告诉 rustc 链接静态库
    println!("cargo:rustc-link-lib=static=calc");
    println!("cargo:rerun-if-changed=../csrc/calc.c");

    // 2. 自动生成绑定
    let bindings = bindgen::Builder::default()
        .header("csrc/calc.h") // 头文件路径
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}