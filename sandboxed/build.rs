use std::path::PathBuf;

fn main() {
    // This is the directory where the `c` library is located.
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    // This is the path to the intermediate object file for our library.
    let obj_path = out_path.join("sandboxed.o");
    // This is the path to the static library file.
    let lib_path = out_path.join("libsandboxed.a");

    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search={}", out_path.to_str().unwrap());

    // Tell cargo to tell rustc to link our `hello` library. Cargo will
    // automatically know it must look for a `libhello.a` file.
    println!("cargo:rustc-link-lib=sandboxed");

    // Run `clang` to compile the `hello.c` file into a `hello.o` object file.
    // Unwrap if it is not possible to spawn the process.
    if !std::process::Command::new("clang")
        .arg("-c")
        .arg("-fembed-bitcode")
        .arg("-o")
        .arg(&obj_path)
        .arg("src/sandbox.c")
        .output()
        .expect("could not spawn `clang`")
        .status
        .success()
    {
        // Panic if the command was not successful.
        panic!("could not compile object file");
    }

    /*if !std::process::Command::new("objcopy")
        .arg("--rename-section")
        .arg(".data=.sandbox-data")
        .arg(&obj_path)
        .output()
        .expect("could not spawn `clang`")
        .status
        .success()
    {
        // Panic if the command was not successful.
        panic!("could not run objcopy");
    }*/

    // Run `ar` to generate the `libhello.a` file from the `hello.o` file.
    // Unwrap if it is not possible to spawn the process.
    if !std::process::Command::new("ar")
        .arg("rcs")
        .arg(lib_path)
        .arg(obj_path)
        .output()
        .expect("could not spawn `ar`")
        .status
        .success()
    {
        // Panic if the command was not successful.
        panic!("could not emit library file");
    }

    let bindings = bindgen::Builder::default()
        .header("src/sandbox.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
