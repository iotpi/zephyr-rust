use core::fmt::Write;
use zephyr_sys::syscalls;

#[no_mangle]
pub extern "C" fn hello_rust() {
    writeln!(&mut std::io::Stdout, "Hello Rust writeln").unwrap();
    {
        const MSG: &str = "Hello from Rust kernel with direct kernel call\n";
        unsafe { syscalls::kernel::k_str_out(MSG.as_ptr() as *mut _, MSG.len()) };
    }
    {
        const MSG: &str = "Hello from Rust kernel with runtime-detect syscall\n";
        unsafe { syscalls::any::k_str_out(MSG.as_ptr() as *mut _, MSG.len()) };
    }
}

#[no_mangle]
pub extern "C" fn hello_rust_user() {
    {
        const MSG: &str = "Hello from Rust userspace with forced user-mode syscall\n";
        unsafe { syscalls::user::k_str_out(MSG.as_ptr() as *mut _, MSG.len()) };
    }
    {
        const MSG: &str = "Hello from Rust userspace with runtime-detect syscall\nNext call will crash if userspace is working.\n";
        unsafe { syscalls::any::k_str_out(MSG.as_ptr() as *mut _, MSG.len()) };
    }

    // This will compile, but crash if CONFIG_USERSPACE is working
    {
        const MSG: &str = "Hello from Rust userspace with direct kernel call\n";
        unsafe { syscalls::kernel::k_str_out(MSG.as_ptr() as *mut _, MSG.len()) };
    }
}
