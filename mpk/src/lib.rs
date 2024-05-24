mod sandbox;

use std::ptr::addr_of;

use bytemuck::AnyBitPattern;
pub use sandbox::Sandbox;

extern "C" {
    static _SANDBOX_START_: libc::c_void;
    static _SANDBOX_END_: libc::c_void;
}

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

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn validate_sandbox_ptr<T>(ptr: *const T) {
    assert!(!ptr.is_null());
    if std::mem::size_of::<T>() != 0 {
        /*unsafe {
            assert!(
                addr_of!(_SANDBOX_START_) <= ptr as *const libc::c_void
                    && addr_of!(_SANDBOX_END_) >= ptr.add(1) as *const libc::c_void,
                "pointer points outside the sandbox"
            );
        }*/
    }
    if (ptr as usize) & (std::mem::align_of::<T>() - 1) != 0 {
        panic!("pointer is misaligned");
    }
}

impl<T: ?Sized> SandboxPtr<T> {
    pub fn new(ptr: *const T) -> Self
    where
        T: Sized,
    {
        validate_sandbox_ptr(ptr);
        SandboxPtr(ptr)
    }

    /// Creates a SandboxPtr without checking its validity.
    ///
    /// # Safety
    ///
    /// If the pointer is converted to a reference, the pointer must be valid and properly aligned.
    pub unsafe fn new_unchecked(ptr: *const T) -> Self {
        SandboxPtr(ptr)
    }

    pub fn get(&self) -> *const T {
        self.0
    }

    pub fn as_ref<'a>(&self, _sandbox: &'a Sandbox) -> &'a T
    where
        T: SandboxSafe,
    {
        unsafe { self.0.as_ref().unwrap() }
    }

    pub fn as_slice(&self, len: usize) -> SandboxPtr<[T]>
    where
        T: Sized,
        T: SandboxSafe,
    {
        unsafe {
            assert!(len == 0 || addr_of!(_SANDBOX_START_) <= self.0 as *const libc::c_void);
            assert!(len == 0 || addr_of!(_SANDBOX_END_) >= self.0.add(len) as *const libc::c_void);
        }
        SandboxPtr(std::ptr::slice_from_raw_parts(self.0, len))
    }
}

impl SandboxPtr<std::ffi::c_char> {
    pub fn from_cstr(s: &'static std::ffi::CStr) -> Self {
        unsafe { Self::new_unchecked(s.as_ptr()) }
    }
}

/// A mutable pointer to a value that lives inside of a sandbox.
#[derive(Clone, Copy)]
pub struct SandboxPtrMut<T: ?Sized>(*mut T);

impl<T: ?Sized> SandboxPtrMut<T> {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn new(ptr: *mut T) -> Self
    where
        T: Sized,
    {
        validate_sandbox_ptr(ptr);
        SandboxPtrMut(ptr)
    }

    pub fn get(&self) -> *mut T {
        self.0
    }

    pub fn as_ref<'a>(&self, _sandbox: &'a Sandbox) -> &'a T
    where
        T: SandboxSafe,
    {
        unsafe { self.0.as_ref().unwrap() }
    }

    pub fn as_mut<'a>(&self, _sandbox: &'a mut Sandbox) -> &'a mut T
    where
        T: SandboxSafe,
    {
        unsafe { self.0.as_mut().unwrap() }
    }

    pub fn as_slice(&self, len: usize) -> SandboxPtrMut<[T]>
    where
        T: Sized,
        T: SandboxSafe,
    {
        unsafe {
            assert!(len == 0 || addr_of!(_SANDBOX_START_) <= self.0 as *const libc::c_void);
            assert!(len == 0 || addr_of!(_SANDBOX_END_) >= self.0.add(len) as *const libc::c_void);
        }
        SandboxPtrMut(std::ptr::slice_from_raw_parts_mut(self.0, len))
    }
}
