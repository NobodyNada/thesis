/// A `SandboxSafe` type is one that lives inside a sandbox, but can be referenced by safe code
/// outside of the sandbox.
///
/// # Safety
///
/// A `SandboxSafe` type must not create UB in safe code, no matter how unsafe, sandboxed code
/// interacts with it. This implies the type has no "holes" in its representation -- any bit
/// pattern is a valid instance of the type. A blanket implementation is provided for types
/// implementing `bytemuck::AnyBitPattern`.
pub unsafe trait SandboxSafe {}
unsafe impl<T: AnyBitPattern> SandboxSafe for T {}
unsafe impl<T: AnyBitPattern> SandboxSafe for [T] {}

/// A pointer to a value that lives inside of a sandbox.
#[derive(Clone, Copy)]
pub struct SandboxPtr<T: ?Sized>(*const T);

impl<T: ?Sized> SandboxPtr<T> {
    pub fn new(ptr: *const T) -> Self
    where
        T: Sized;

    /// Creates a SandboxPtr without checking its validity.
    ///
    /// # Safety
    ///
    /// If the pointer is converted to a reference, the pointer must be valid and properly aligned.
    pub unsafe fn new_unchecked(ptr: *const T) -> Self;

    pub fn get(&self) -> *const T;
    pub fn as_ref<'a>(&self, _sandbox: &'a Sandbox) -> &'a T
    where
        T: SandboxSafe;
    pub fn as_slice(&self, len: usize) -> SandboxPtr<[T]>
    where
        T: Sized,
        T: SandboxSafe;
}

impl SandboxPtr<std::ffi::c_char> {
    pub fn from_cstr(s: &'static std::ffi::CStr) -> Self;
}

/// A mutable pointer to a value that lives inside of a sandbox.
#[derive(Clone, Copy)]
pub struct SandboxPtrMut<T: ?Sized>(*mut T);

impl<T: ?Sized> SandboxPtrMut<T> {
    pub fn new(ptr: *mut T) -> Self
    where
        T: Sized;
    pub unsafe fn new_unchecked(ptr: *mut T) -> Self;
    pub fn get(&self) -> *mut T;
    pub fn as_ref<'a>(&self, _sandbox: &'a Sandbox) -> &'a T
    where
        T: SandboxSafe;
    pub fn as_mut<'a>(&self, _sandbox: &'a mut Sandbox) -> &'a mut T
    where
        T: SandboxSafe;
    pub fn as_slice(&self, len: usize) -> SandboxPtrMut<[T]>
    where
        T: Sized,
        T: SandboxSafe;
}

#[allow(unused)]
pub struct Sandbox { ... }
unsafe impl Send for Sandbox {}

impl Sandbox {
    pub const fn new() -> Sandbox;

    /// Calls a function within the sandbox.
    ///
    /// # Safety
    ///
    /// The provided function must not reference data that lives outside the sandbox.
    pub unsafe fn call<T, F: FnOnce() -> T + 'static>(&mut self, f: F) -> T;
}
