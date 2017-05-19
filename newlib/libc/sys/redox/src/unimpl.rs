use syscall;
use std::io::{stderr, Write};
use libc::{c_uint, c_int, c_char, gid_t, uid_t, c_void, c_long, mode_t};
use syscall::error::{Error, EACCES, EPERM, EINVAL};
use std::ptr::null;

type clock_t = c_long;

macro_rules! UNIMPL {
    // Call with arguments and return value
    ($func:ident, $err:ident) => {{
         let err = Error::new($err);
         writeln!(stderr(), "unimplemented: {}: {}",
             stringify!($func), err).unwrap();
         Err(err)
    }};
}

#[no_mangle]
pub extern "C" fn alarm(_seconds: c_uint) -> c_uint {
    writeln!(stderr(), "unimplemented: alarm").unwrap();
    0
}

libc_fn!(_chown(_path: *mut c_char, _order: uid_t, _group: gid_t) -> c_int {
    UNIMPL!(_chown, EACCES)
});

// XXX variadic
libc_fn!(_fcntl(_file: c_int, _cmd: c_int) -> c_int {
    UNIMPL!(_fcntl, EACCES)
});

// XXX return value pointer type
libc_fn!(_gethostbyname(_name: *const c_char) -> *const c_void {
    Ok(null())
});

libc_fn!(_getdtablesize() -> c_int {
    Ok(65536)
});

// XXX return value pointer type
libc_fn!(_getgrnam(_name: *const c_char) -> *const c_void {
    Ok(null())
});

// XXX return value pointer type
libc_fn!(_getgrgid(_gid: gid_t) -> *const c_void {
    Ok(null())
});

// XXX return value pointer type
libc_fn!(_getpwnam(_name: *const c_char) -> *const c_void {
    Ok(null())
});

// XXX return value pointer type
libc_fn!(_getpwuid(_gid: uid_t) -> *const c_void {
    Ok(null())
});

// XXX variadic
libc_fn!(_ioctl(_file: c_int, _request: c_int) -> c_int {
    UNIMPL!(_ioctl, EINVAL)
});

// TODO: Actually implement lstat, it currently just calls stat
#[no_mangle]
pub unsafe extern "C" fn lstat(path: *const c_char, sbuf: *mut syscall::Stat) -> c_int {
    ::file::_stat(path, sbuf)
}

libc_fn!(_link(_old: *const c_char, _new: *const c_char) -> c_int {
    UNIMPL!(_link, EPERM)
});

libc_fn!(sysconf(_name: c_int) -> c_long {
    UNIMPL!(sysconf, EINVAL)
});

// XXX type of argument pointer
libc_fn!(_times(_buf: *mut c_void) -> clock_t {
    UNIMPL!(_times, EINVAL)
});

#[no_mangle]
pub unsafe extern "C" fn _umask(_mode: mode_t) -> mode_t {
    // All permissions granted
    0o777
}

// XXX type of argument pointer
libc_fn!(_utime(_filename: *const c_char, _times: *mut c_void) -> c_int {
    UNIMPL!(_utime, EACCES)
});

#[no_mangle]
pub unsafe extern "C" fn vfork() -> c_int {
    ::process::_fork()
}
