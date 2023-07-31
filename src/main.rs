#![no_main]
#![no_std]

use core::arch::global_asm;
use core::panic::PanicInfo;

global_asm!(r#"
.section ".text.init"
.globl _start
_start:
  li sp, 0x80003000
  j __start_rust
"#);

#[no_mangle]
pub fn __start_rust() -> ! {
  let uart0 = 0x10000000 as *mut u8;
  for c in b"Hello from Rust!".iter() {
    unsafe {
      *uart0 = *c as u8;
    }
  }
  loop {}
}

#[panic_handler]
#[no_mangle]
fn panic(_panic: &PanicInfo<'_>) -> ! {
  loop {}
}

#[no_mangle]
pub fn abort() -> ! {
  loop {}
}
