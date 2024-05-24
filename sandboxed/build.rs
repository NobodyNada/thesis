use std::{ffi::OsStr, fs::read_dir, path::PathBuf};

fn compile(src: impl AsRef<OsStr>, out: impl AsRef<OsStr>) {
    #[cfg(feature = "cet")]
    let cet = Some("-fcf-protection=full");

    #[cfg(not(feature = "cet"))]
    let cet = None::<&str>;

    let process = std::process::Command::new("clang")
        .arg("-O3")
        .arg("-c")
        .args(cet)
        .arg("-o")
        .arg(&out)
        .arg(src)
        .output()
        .expect("could not spawn `clang`");
    if !process.status.success() {
        // Panic if the command was not successful.
        panic!(
            "could not compile object file: {}",
            String::from_utf8_lossy(&process.stderr)
        );
    }
}

fn main() {
    // This is the directory where the `c` library is located.
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    let mut objs = Vec::<PathBuf>::new();

    // This is the path to the intermediate object file for our library.
    let obj_path = out_path.join("sandboxed.o");
    // This is the path to the static library file.
    let lib_path = out_path.join("libsandboxed.a");

    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search={}", out_path.to_str().unwrap());

    // Tell cargo to tell rustc to link our `hello` library. Cargo will
    // automatically know it must look for a `libhello.a` file.
    println!("cargo:rustc-link-lib=sandboxed");

    // Run `clang` to compile the `sandboxed.c` file into a `sandboxed.o` object file.

    compile("src/sandbox.c", &obj_path);
    objs.push(obj_path);

    for file in read_dir("src/cmark").unwrap().skip(1) {
        let file = file.unwrap();
        if file.file_name().to_string_lossy().ends_with(".c") {
            let obj_path = out_path.join(file.path().with_extension("o").file_name().unwrap());
            compile(file.path(), &obj_path);
            objs.push(obj_path);
        }
    }

    // Run `ar` to generate the `sandboxed.a` file from the `sandboxed.o` file.
    // Unwrap if it is not possible to spawn the process.
    if !std::process::Command::new("ar")
        .arg("rcs")
        .arg(lib_path)
        .args(objs)
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
