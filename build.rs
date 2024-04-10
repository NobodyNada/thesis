use std::{fs::File, io::Write, path::PathBuf};

fn main() {
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    let linker_script_path = out_path.join("libsandboxed.ld");
    let linker_script = r"
SECTIONS {
  .data : ALIGN(CONSTANT(MAXPAGESIZE)) {
    _SANDBOX_START_ = .;
    *libsandboxed.a:*(.data )
    . = ALIGN(CONSTANT(MAXPAGESIZE));
    _SANDBOX_END_ = .;
    *(.data)
  }
} INSERT BEFORE .bss
";

    File::create(&linker_script_path)
        .unwrap()
        .write_all(linker_script.as_bytes())
        .unwrap();

    println!(
        "cargo:rustc-link-arg=-T{}",
        linker_script_path.to_string_lossy()
    );
    println!("cargo:rustc-link-arg=-fuse-ld=lld")
}
