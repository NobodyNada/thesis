#[allow(unused)]
use super::*;
#[allow(unused)]
use std::{arch::asm, mem::ManuallyDrop, ptr::null_mut};

#[allow(unused)]
pub struct Sandbox {
    pkey: Option<std::io::Result<i32>>,
    stack: *mut libc::c_void,
}

unsafe impl Send for Sandbox {}

#[cfg(feature = "mpk")]
const SANDBOX_STACK_SIZE: usize = 1 << 23; // 8 MiB

#[cfg(feature = "mpk")]
static mut RETURNSTACK: *mut libc::c_void = null_mut();

#[allow(unused)]
const PKEY_DISABLE_ACCESS: u32 = 0x1;
#[allow(unused)]
const PKEY_DISABLE_WRITE: u32 = 0x2;

impl Sandbox {
    pub const fn new() -> Sandbox {
        Sandbox {
            pkey: None,
            stack: null_mut(),
        }
    }

    /// Calls a function within the sandbox.
    ///
    /// # Safety
    ///
    /// The provided function must not reference data that lives outside the sandbox.
    #[cfg(feature = "mpk")]
    pub unsafe fn call<T, F: FnOnce() -> T + 'static>(&mut self, f: F) -> T {
        if self.init().is_none() {
            // MPK is not available, so just call the function directly
            return f();
        };

        assert_eq!(RETURNSTACK, null_mut());

        // Write the closure to the sandbox stack.
        let sp = self.stack.byte_add(SANDBOX_STACK_SIZE);
        let sp = sp.cast::<SandboxArgs<T, F>>().sub(1);
        sp.write(SandboxArgs {
            f: ManuallyDrop::new(f),
        });

        let oldpkru = rdpkru();
        let newpkru = pkru_set(oldpkru, 11, PKEY_DISABLE_WRITE);

        asm!(
            "
        mov [rip + {oldstack}], rsp  // Save old stack pointer
        mov rsp, rdi                 // Switch to new stack
        wrpkru                       // Switch to sandbox protection
        call {_sandbox_call}         // Call sandboxed function
        
        mov rax, r12
        xor rcx, rcx
        xor rdx, rdx
        wrpkru                       // Restore previous protection
        mov rsp, [rip + {oldstack}]  // Switch to old stack
        ",
            oldstack = sym RETURNSTACK,
            in ("rdi") sp,          // argument 0 to _sandbox_call
            in ("rax") newpkru,
            in ("rcx") 0,
            inout ("rdx") 0 => _,
            in ("r12") oldpkru,     // need a callee-preserved register
            _sandbox_call = sym _sandbox_call::<T, F>,
            clobber_abi("sysv64")
        );

        RETURNSTACK = null_mut();
        ManuallyDrop::into_inner(sp.read().result)
    }

    /// Calls a function within the sandbox.
    ///
    /// # Safety
    ///
    /// The provided function must not reference data that lives outside the sandbox.
    #[cfg(not(feature = "mpk"))]
    pub unsafe fn call<T, F: FnOnce() -> T + 'static>(&mut self, f: F) -> T {
        f()
    }

    #[cfg(feature = "mpk")]
    fn init(&mut self) -> Option<i32> {
        if let Some(result) = &self.pkey {
            if let Ok(pkey) = result {
                return Some(*pkey);
            } else {
                return None;
            }
        }

        let result = (|| {
            let pkey = pkey_alloc(0, 0);
            if pkey < 0 {
                return Err(std::io::Error::last_os_error());
            }

            unsafe {
                let sandbox_start = std::ptr::addr_of!(_SANDBOX_START_) as *mut libc::c_void;
                let sandbox_end = std::ptr::addr_of!(_SANDBOX_END_) as *mut libc::c_void;

                let err = pkey_mprotect(
                    sandbox_start,
                    sandbox_end as usize - sandbox_start as usize,
                    libc::PROT_READ | libc::PROT_WRITE,
                    pkey,
                );
                if err < 0 {
                    return Err(std::io::Error::last_os_error());
                }

                let stack = libc::mmap(
                    null_mut(),
                    SANDBOX_STACK_SIZE,
                    libc::PROT_READ | libc::PROT_WRITE,
                    libc::MAP_ANONYMOUS | libc::MAP_PRIVATE,
                    -1,
                    0,
                );
                if stack == libc::MAP_FAILED {
                    return Err(std::io::Error::last_os_error());
                }

                let err = pkey_mprotect(
                    stack,
                    SANDBOX_STACK_SIZE,
                    libc::PROT_READ | libc::PROT_WRITE,
                    pkey,
                );
                if err < 0 {
                    return Err(std::io::Error::last_os_error());
                }

                self.stack = stack;
            }

            Ok(pkey)
        })();

        self.pkey = Some(result);
        match self.pkey.as_ref().unwrap() {
            Ok(pkey) => Some(*pkey),
            Err(e) => {
                eprintln!("warning: could not initialize MPK, proceeding without sandbox: {e}");
                None
            }
        }
    }
}

#[cfg(feature = "mpk")]
#[repr(align(16))]
union SandboxArgs<T, F: FnOnce() -> T + 'static> {
    f: ManuallyDrop<F>,
    result: ManuallyDrop<T>,
}

#[cfg(feature = "mpk")]
unsafe extern "sysv64" fn _sandbox_call<T, F: FnOnce() -> T + 'static>(
    args: *mut SandboxArgs<T, F>,
) {
    let f = ManuallyDrop::into_inner(args.read().f);
    let result = f();
    args.write(SandboxArgs {
        result: ManuallyDrop::new(result),
    });
}

#[cfg(feature = "mpk")]
#[inline(always)]
fn rdpkru() -> u32 {
    unsafe {
        let mut result: u32;
        asm!("rdpkru", out("eax") result, in("ecx") 0, out("edx") _, options(nomem, nostack));
        result
    }
}

#[inline(always)]
#[allow(unused)]
#[cfg(feature = "mpk")]
unsafe fn wrpkru(value: u32) {
    asm!("wrpkru", in("eax") value, options(nostack));
}

#[cfg(feature = "mpk")]
fn pkru_set(pkru: u32, pkey: i32, prot: u32) -> u32 {
    assert!((0..0x10).contains(&pkey));
    assert!((..0x4).contains(&prot));
    let shift = pkey as u32 * 2;
    let mask = 0x3 << shift;
    (pkru & !mask) | (prot << shift)
}

#[cfg(feature = "mpk")]
fn pkey_alloc(flags: u32, access_rights: u32) -> i32 {
    unsafe { libc::syscall(libc::SYS_pkey_alloc, flags, access_rights) as i32 }
}

#[cfg(feature = "mpk")]
unsafe fn pkey_mprotect(addr: *mut libc::c_void, len: usize, prot: i32, pkey: i32) -> i32 {
    libc::syscall(libc::SYS_pkey_mprotect, addr, len, prot, pkey) as i32
}
