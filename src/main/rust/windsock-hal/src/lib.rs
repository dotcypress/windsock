#![no_std]
#![allow(non_camel_case_types)]

pub extern crate embedded_hal as hal;
pub extern crate nb;

pub use crate::{modify_reg, read_reg};
pub use windsock_pac::arch::interrupt;
pub use windsock_pac::*;

pub mod color;
pub mod gpio;
pub mod prelude;
pub mod serial;
pub mod time;
pub mod timer;

use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};

use crate::system::Instance;
use crate::time::{Hertz, NanoSecond};

#[allow(dead_code)]
pub struct System {
  instance: Instance,
}

pub trait SystemExt {
  fn system(self) -> System;
}

impl SystemExt for Instance {
  fn system(self) -> System {
    System::new(self)
  }
}

impl System {
  pub fn new(instance: Instance) -> Self {
    modify_reg!(system, instance, CONTROL, PANIC: 0x00);
    System { instance }
  }

  pub fn enable_leds(&mut self) {
    modify_reg!(system, self.instance, CONTROL, ENABLE_LEDS: 0x01);
  }

  pub fn disable_leds(&mut self) {
    modify_reg!(system, self.instance, CONTROL, ENABLE_LEDS: 0x00);
  }

  pub fn set_led_color(&mut self, led_idx: u8, color: color::RgbColor) {
    let color =
      (color.blue as u32 & 0xF) << 8 | (color.green as u32 & 0xF) << 4 | color.red as u32 & 0xF;
    match led_idx {
      0 => modify_reg!(system, self.instance, LEDA, COLOR0: color),
      1 => modify_reg!(system, self.instance, LEDA, COLOR1: color),
      2 => modify_reg!(system, self.instance, LEDB, COLOR2: color),
      _ => modify_reg!(system, self.instance, LEDB, COLOR3: color),
    }
  }

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
    modify_reg!(system, SYSTEM, CONTROL, PANIC: 0x01);
  };
  loop {
    atomic::compiler_fence(Ordering::SeqCst);
  }
}
