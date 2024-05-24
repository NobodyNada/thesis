#![cfg_attr(feature = "bench", feature(test))]

fn main() {
    println!("{}", call_sandboxed_function(5));
}

#[inline(never)]
fn call_sandboxed_function(x: i32) -> i32 {
    let mut sandbox = sandboxed::SANDBOXED.lock().unwrap();
    sandbox.sandboxed(x)
}

#[cfg(test)]
#[cfg(feature = "bench")]
mod bench {
    use std::ffi::CString;

    use mpk::SandboxPtr;

    extern crate test;
    use test::Bencher;

    #[bench]
    fn nop(b: &mut Bencher) {
        let mut sandbox = sandboxed::SANDBOXED.lock().unwrap();
        b.iter(|| sandbox.nop());
    }

    #[bench]
    fn cmark_simple(b: &mut Bencher) {
        let mut sandbox = sandboxed::SANDBOXED.lock().unwrap();

        b.iter(|| {
            let document = c"Hello, *world*";
            let document = sandbox.cmark_parse_document(
                SandboxPtr::from_cstr(document),
                document.to_bytes().len(),
                sandboxed::CMARK_OPT_DEFAULT as i32,
            );
            let html = sandbox.cmark_render_html(document, sandboxed::CMARK_OPT_DEFAULT as i32);

            html
        });
    }

    #[bench]
    fn cmark_large(b: &mut Bencher) {
        let mut sandbox = sandboxed::SANDBOXED.lock().unwrap();
        let document = CString::new(include_bytes!("./progit-bench.md")).unwrap();
        let len = document.to_bytes().len();
        let document = unsafe { SandboxPtr::new_unchecked(document.as_ptr()) };

        b.iter(|| {
            let document =
                sandbox.cmark_parse_document(document, len, sandboxed::CMARK_OPT_DEFAULT as i32);
            let html = sandbox.cmark_render_html(document, sandboxed::CMARK_OPT_DEFAULT as i32);

            html
        });
    }
}
