
#![no_std]
#![no_main]

use core::arch::asm;
use core::fmt::{self, Write};
use core::panic::PanicInfo;

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut a0 = args[0];

    unsafe {
        asm!(
            "ecall",
            inlateout("a0") a0,
            in("a1") args[1],
            in("a2") args[2],
            in("a7") id,
            options(nostack),
        );
    }

    a0 as isize
}

pub fn sys_exit(xstate: i32) -> isize {
    syscall(SYSCALL_EXIT, [xstate as usize, 0, 0])
}

pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(
        SYSCALL_WRITE,
        [fd, buffer.as_ptr() as usize, buffer.len()],
    )
}

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let result = sys_write(1, s.as_bytes());

        if result < 0 || result as usize != s.len() {
            return Err(fmt::Error);
        }

        Ok(())
    }
}

pub fn print(args: fmt::Arguments<'_>) {
    let mut stdout = Stdout;
    stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::print(core::format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {
        $crate::print!("{}\n", core::format_args!($($arg)*))
    };
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello, world!");

    sys_exit(9);
    loop {
        core::hint::spin_loop();
    }
}
