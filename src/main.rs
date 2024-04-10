fn main() {
    let mut sandbox = sandboxed::SANDBOXED.lock().unwrap();
    println!("{}", unsafe { sandbox.sandboxed(5) });
}
