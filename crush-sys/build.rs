use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=../crush/libcrush.h");
    println!("cargo:rerun-if-changed=../crush/crush.h");
    println!("cargo:rerun-if-changed=../crush/builder.h");
    println!("cargo:rerun-if-changed=../crush/mapper.h");
    println!("cargo:rerun-if-changed=../crush/helpers.h");
    println!("cargo:rerun-if-changed=../crush/hash.h");
    
    // Compile CRUSH C library
    cc::Build::new()
        .file("../crush/crush.c")
        .file("../crush/builder.c")
        .file("../crush/mapper.c")
        .file("../crush/hash.c")
        .file("../crush/helpers.c")
        .include("..")
        .compile("crush");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-I..")
        .clang_arg("-I.")
        .clang_arg("-I/usr/include")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Disable doc tests for generated bindings
        .generate_comments(false)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
