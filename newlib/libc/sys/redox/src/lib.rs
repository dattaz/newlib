#![no_std]
#![feature(collections, lang_items, core_intrinsics, compiler_builtins_lib, linkage, drop_types_in_const, const_fn, alloc)]

extern crate syscall;
#[macro_use]
extern crate collections;
extern crate compiler_builtins;
extern crate byteorder;
extern crate alloc;

use core::{ptr, mem, intrinsics, slice};
use collections::Vec;
use ::types::{c_int, c_void, c_char, size_t};

#[macro_use]
mod macros;
mod types;
mod dns;
pub mod process;
pub mod file;
pub mod folder;
pub mod time;
pub mod unimpl;
pub mod redox;
pub mod socket;
pub mod hostname;

extern {
    // Newlib uses this function instead of just a global to support reentrancy
    pub fn __errno() -> *mut c_int;
    pub fn malloc(size: size_t) -> *mut c_void;
    pub fn free(ptr: *mut c_void);
    pub fn strlen(s: *const c_char) -> size_t;
    pub fn __libc_fini_array();
    pub static mut environ: *mut *mut c_char;
}

pub unsafe fn cstr_to_slice<'a>(buf: *const c_char) -> &'a [u8] {
    slice::from_raw_parts(buf as *const u8, ::strlen(buf) as usize)
}
pub unsafe fn cstr_to_slice_mut<'a>(buf: *const c_char) ->  &'a mut [u8] {
    slice::from_raw_parts_mut(buf as *mut u8, ::strlen(buf) as usize)
}

pub fn file_read_all<T: AsRef<[u8]>>(path: T) -> syscall::Result<Vec<u8>> {
    let fd = syscall::open(path, syscall::O_RDONLY)?;

    let mut st = syscall::Stat::default();
    syscall::fstat(fd, &mut st)?;
    let size = st.st_size as usize;

    let mut buf = Vec::with_capacity(size);
    unsafe { buf.set_len(size) };
    syscall::read(fd, buf.as_mut_slice())?;

    syscall::close(fd)?;

    Ok(buf)
}

#[no_mangle]
pub unsafe extern "C" fn __errno_location() -> *mut c_int {
    __errno()
}

#[lang = "panic_fmt"]
#[linkage = "weak"]
#[no_mangle]
pub extern fn rust_begin_panic(_msg: core::fmt::Arguments,
                               _file: &'static str,
                               _line: u32) -> ! {
    unsafe { intrinsics::abort() }
}

libc_fn!(unsafe initialize_standard_library() {
    let mut buf = file_read_all("env:").unwrap();
    let size = buf.len();
    buf.push(0);

    let mut vars = Vec::new();

    vars.push(&mut buf[0] as *mut u8 as *mut c_char);
    for i in 0..size {
        if buf[i] == b'\n' {
            if i != size - 1 {
                vars.push(&mut buf[i + 1] as *mut u8 as *mut c_char);
            }
            buf[i] = b'\0';
        }
    }
    vars.push(ptr::null_mut());

    environ = vars.as_mut_ptr();
    mem::forget(vars); // Do not free memory
    mem::forget(buf);
});
