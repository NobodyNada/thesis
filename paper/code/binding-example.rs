impl Sandboxed {
    pub fn foo(
        &mut self,
        p: mpk::SandboxPtrMut<::std::os::raw::c_int>,
    ) -> mpk::SandboxPtr<::std::os::raw::c_int> {
        extern "C" {
            fn foo(p: *mut ::std::os::raw::c_int) -> *const ::std::os::raw::c_int;
        }
        unsafe { mpk::SandboxPtr::new(self.0.call(move || foo(p.get()))) }
    }
}
