#![no_std]
#![no_main]

use core::arch::asm;
use core::panic;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print("hello world\n");
    exit(0);
}

pub fn print(str: &str) {
    const STDOUT_FILENO: u32 = 1;
    unsafe {
        write(STDOUT_FILENO, str.as_ptr(), str.len());
    }
}

pub fn eprint(str: &str) {
    const STDERR_FILENO: u32 = 2;
    unsafe {
        write(STDERR_FILENO, str.as_ptr(), str.len());
    }
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub unsafe fn write(fd: u32, buf: *const u8, count: usize) {
    const SYSCALL_NUMBER: u64 = 1;
    asm!(
        "syscall",
        in("rax") SYSCALL_NUMBER,
        in("rdi") fd,
        in("rsi") buf,
        in("rdx") count,
        options(nostack)
    );
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub fn exit(code: i32) -> ! {
    const SYSCALL_NUMBER: u64 = 60;
    unsafe {
        asm!(
            "syscall",
            in("rax") SYSCALL_NUMBER,
            in("rdi") code,
            options(noreturn)
        )
    }
}

#[panic_handler]
fn panic(_info: &panic::PanicInfo) -> ! {
    eprint("panic\n");
    exit(1);
}
