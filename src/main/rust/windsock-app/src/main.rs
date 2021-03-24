#![no_std]
#![no_main]

use core::fmt::Write;

use windsock_hal::color::*;
use windsock_hal::gpioa::GPIOA;
use windsock_hal::prelude::*;
use windsock_hal::rt::entry;
use windsock_hal::serial;
use windsock_hal::system::SYSTEM;
use windsock_hal::timer1::TIMER1;
use windsock_hal::uart1::UART1;

#[entry]
fn main() -> ! {
  let gpioa = GPIOA::take().unwrap().split();
  let mut sys = SYSTEM::take().unwrap().system();
  let mut uart = UART1::take().unwrap().serial(serial::Config::default());
  let mut timer = TIMER1::take().unwrap().timer();

  let btn_1 = gpioa.pa31;
  let btn_2 = gpioa.pa30;
  let mut led_0 = gpioa.pa0.into_output();
  let mut led_1 = gpioa.pa1.into_output();
  let mut led_2 = gpioa.pa2.into_output();
  let mut led_3 = gpioa.pa3.into_output();

  led_0.set_low().ok();
  led_1.set_high().ok();
  led_2.set_low().ok();
  led_3.set_high().ok();

  sys.enable_leds();
  write!(uart, "WindSock 0.1\r\n").ok();

  uart.rx().listen();
  unsafe {
    windsock_hal::arch::register::mstatus::set_mie();
    windsock_hal::arch::register::mie::set_mext();
  }

  loop {
    led_0.toggle().ok();
    led_1.toggle().ok();
    led_2.toggle().ok();
    led_3.toggle().ok();

    if btn_1.is_high().unwrap() {
      panic!("Hello panic");
    }

    if btn_2.is_high().unwrap() {
      timer.delay(300.ms());
    }

    timer.delay(300.ms());
  }
}

#[export_name = "MachineExternal"]
fn uart_interrupt() {
  let mut sys = unsafe { SYSTEM::conjure().system() };
  let mut uart = unsafe { UART1::conjure().serial(serial::Config::default()) };
  match uart.read() {
    Ok(byte) => {
      uart.write(byte).ok();
      sys.set_led_color(0, RgbColor::new(byte >> 1, 0, 0));
      sys.set_led_color(1, RgbColor::new(0, byte >> 5, 0));
    }
    Err(windsock_hal::nb::Error::WouldBlock) => {}
    Err(_) => {
      panic!("Serial fault");
    }
  }
}
