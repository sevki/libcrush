use std::env;
use std::path::PathBuf;

fn main() {
    let cargo_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    println!("cargo:rerun-if-changed={cargo_dir}/crush/libcrush.h");
    println!("cargo:rerun-if-changed={cargo_dir}/crush/crush.h");
    println!("cargo:rerun-if-changed={cargo_dir}/crush/builder.h");
    println!("cargo:rerun-if-changed={cargo_dir}/crush/mapper.h");
    println!("cargo:rerun-if-changed={cargo_dir}/crush/helpers.h");
    println!("cargo:rerun-if-changed={cargo_dir}/crush/hash.h");

    // Compile CRUSH C library
    cc::Build::new()
        .file(format!("{}/crush/crush.c", cargo_dir))
        .file(format!("{}/crush/builder.c", cargo_dir))
        .file(format!("{}/crush/mapper.c", cargo_dir))
        .file(format!("{}/crush/hash.c", cargo_dir))
        .file(format!("{}/crush/helpers.c", cargo_dir))
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
