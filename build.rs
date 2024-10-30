fn main() {
    build_cxx();
    build_csharp();
}

fn build_cxx() {
    use std::env;
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(cbindgen::Language::C)
        .with_cpp_compat(true)
        .include_item("AlignN")
        .include_item("VecInfo")
        .include_item("WPoint2")
        .include_item("WPoint3")
        .include_item("WPoint4")
        .with_include_guard("F3L_WRAPPER_C_H")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("generated/f3l_wrapper_c.g.h");
}

fn build_csharp() {
    csbindgen::Builder::new()
        .input_extern_file("src/lib.rs")
        .input_extern_file("src/points.rs")
        .always_included_types(["AlignN", "VecInfo", "WPoint2", "WPoint3", "WPoint4"])
        .csharp_class_name("WrapperC")
        // .csharp_dll_name("f3l_wrapper_c")
        .csharp_namespace("F3l")
        .generate_csharp_file("generated/f3l_wrapper_c.g.cs")
        .expect("Unable to generate bindings.");
}