#![no_std]
#![no_main]

use core::arch::asm;
use core::panic;

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    print(b"hello world\n");
    exit(0);
}

pub fn print(s: &[u8]) {
    const STDOUT_FILENO: u32 = 1;
    unsafe {
        write(STDOUT_FILENO, s.as_ptr(), s.len());
    }
}

#[cfg(target_os = "linux")]
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

#[cfg(target_os = "linux")]
pub unsafe fn exit(code: i32) -> ! {
    const SYSCALL_NUMBER: u64 = 60;
    asm!(
        "syscall",
        in("rax") SYSCALL_NUMBER,
        in("rdi") code,
        options(noreturn)
    )
}

#[panic_handler]
fn panic(_info: &panic::PanicInfo) -> ! {
    unsafe { exit(1) }
}
