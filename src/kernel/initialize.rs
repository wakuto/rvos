use crate::{println, print};
use riscv::register::*;

pub fn initialize() {
  unsafe {riscv::interrupt::disable()};
  println!("Hello from Rust!");
  println!("mtvec = {:#x}", mtvec::read().bits());
  unsafe {
    mtvec::write(trapvec as usize, mtvec::TrapMode::Direct);
  }
  println!("mtvec = {:#x}", mtvec::read().bits());
}

#[no_mangle]
pub fn trapvec() {
  println!("here is trapvec");
}

