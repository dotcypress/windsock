#![no_std]
#![allow(non_camel_case_types)]

pub extern crate embedded_hal as hal;
pub extern crate nb;

pub use crate::{modify_reg, read_reg, write_reg};
pub use windsock_pac::arch::interrupt;
pub use windsock_pac::*;

pub mod gpio;
pub mod prelude;
pub mod serial;
pub mod time;
pub mod timer;

use crate::time::{Hertz, NanoSecond};
use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};

pub struct Device;

impl Device {
  pub fn clk_frequency() -> Hertz {
    unsafe { Hertz(read_reg!(crate::system, SYSTEM, CLOCK)) }
  }

  pub fn clk_period() -> NanoSecond {
    unsafe { NanoSecond(read_reg!(crate::system, SYSTEM, PERIOD)) }
  }
}

#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  unsafe {
    write_reg!(system, SYSTEM, PANIC, 0x00);
  };
  loop {
    atomic::compiler_fence(Ordering::SeqCst);
  }
}
