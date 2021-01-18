#![no_std]
#![no_main]

use core::fmt::Write;

use windsock_hal::gpioa::GPIOA;
use windsock_hal::prelude::*;
use windsock_hal::read_reg;
use windsock_hal::rgb::RGB;
use windsock_hal::rt::entry;
use windsock_hal::serial;
use windsock_hal::timer1::TIMER1;
use windsock_hal::uart1::UART1;

#[entry]
fn main() -> ! {
  let gpioa = GPIOA::take().unwrap().split();
  let mut uart = UART1::take().unwrap().serial(serial::Config::default());
  let mut timer = TIMER1::take().unwrap().timer();

  write!(uart, "WindSock 0.1\r\n").ok();

  let mut led_0 = gpioa.pa0.into_output();
  let mut led_1 = gpioa.pa1.into_output();
  let mut led_2 = gpioa.pa2.into_output();
  let mut led_3 = gpioa.pa3.into_output();

  let btn_1 = gpioa.pa31;

  uart.rx().listen();
  unsafe {
    windsock_hal::arch::register::mstatus::set_mie();
    windsock_hal::arch::register::mie::set_mext();
  }

  led_0.set_low().ok();
  led_1.set_high().ok();
  led_2.set_low().ok();
  led_3.set_high().ok();

  let rgb = RGB::take().unwrap();

  loop {
    led_0.toggle().ok();
    led_1.toggle().ok();
    led_2.toggle().ok();
    led_3.toggle().ok();

    if btn_1.is_high().unwrap() {
      panic!("Hello panic");
    }

    write!(
      uart,
      "{} {} {} {}\r\n",
      read_reg!(windsock_hal::rgb, rgb, SENSOR, RED),
      read_reg!(windsock_hal::rgb, rgb, SENSOR, GREEN),
      read_reg!(windsock_hal::rgb, rgb, SENSOR, BLUE),
      read_reg!(windsock_hal::rgb, rgb, SENSOR, LUMA),
    )
    .ok();

    timer.delay(50.ms());
  }
}

#[export_name = "MachineExternal"]
fn uart_interrupt() {
  let mut uart = unsafe { UART1::conjure().serial(serial::Config::default()) };
  match uart.read() {
    Ok(byte) => {
      uart.write(byte).ok();
    }
    Err(windsock_hal::nb::Error::WouldBlock) => {}
    Err(_) => {
      panic!("Serial fault");
    }
  }
}
