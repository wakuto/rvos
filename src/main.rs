#![no_main]
#![no_std]

use core::arch::global_asm;
use core::panic::PanicInfo;

mod kernel;

global_asm!(r#"
.section ".text.init"
.globl _start
_start:
  li sp, 0x80010000
  j __start_rust
"#);

#[no_mangle]
pub fn __start_rust() -> ! {
  kernel::initialize::initialize();
  loop {}
}

#[panic_handler]
#[no_mangle]
fn panic(_panic: &PanicInfo<'_>) -> ! {
  println!("panic! : {}", _panic);
  loop {}
}

#[no_mangle]
pub fn abort() -> ! {
  println!("abort!");
  loop {}
}
