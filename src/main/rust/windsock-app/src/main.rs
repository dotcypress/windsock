#![no_std]
#![no_main]

use core::fmt::Write;

use windsock_hal::color::RgbColor;
use windsock_hal::gpioa::GPIOA;
use windsock_hal::rt::entry;
use windsock_hal::serial;
use windsock_hal::system::SYSTEM;
use windsock_hal::timer1::TIMER1;
use windsock_hal::uart1::{self, UART1};
use windsock_hal::System;
use windsock_hal::{interrupt, prelude::*};

use ushell::{autocomplete::*, history::*, *};

const MAX_COMMAND_LEN: usize = 16;
const HISTORY_SIZE: usize = 4;
const AUTOCOMPLETE_SIZE: usize = 3;

const SHELL_PROMPT: &str = "#> ";
const CR: &str = "\r\n";
const HELP: &str = "\r\n\
\x1b[31mWindSock Shell\x1b[0m\r\n\r\n\
USAGE:\r\n\
\tcommand [arg]\r\n\r\n\
COMMANDS:\r\n\
\ton        Switch led on\r\n\
\toff       Switch led off\r\n\
\tstatus    Get led status\r\n\
\tclear     Clear screen\r\n\
\thelp      Print this message\r\n
";

struct Env {
    led_on: bool,
    sys: System,
    shell: UShell<
        serial::Serial<uart1::Instance>,
        StaticAutocomplete<{ AUTOCOMPLETE_SIZE }>,
        LRUHistory<{ MAX_COMMAND_LEN }, { HISTORY_SIZE }>,
        { MAX_COMMAND_LEN },
    >,
}

static mut ENV: Option<Env> = None;

#[entry]
fn main() -> ! {
    let gpioa = GPIOA::take().unwrap().split();

    let mut sys = SYSTEM::take().unwrap().system();
    sys.enable_leds();

    let pmod_btn_1 = gpioa.pa31;
    let pmod_btn_2 = gpioa.pa30;
    let mut pmod_led_0 = gpioa.pa0.into_output();
    let mut pmod_led_1 = gpioa.pa1.into_output();
    let mut pmod_led_2 = gpioa.pa2.into_output();
    let mut pmod_led_3 = gpioa.pa3.into_output();

    pmod_led_0.set_low().ok();
    pmod_led_1.set_high().ok();
    pmod_led_2.set_low().ok();
    pmod_led_3.set_high().ok();

    let mut timer = TIMER1::take().unwrap().timer();

    let autocomplete = StaticAutocomplete(["clear", "help", "status"]);
    let history = LRUHistory::default();

    let mut uart = UART1::take().unwrap().serial(serial::Config::default());
    uart.rx().listen();

    let mut shell = UShell::new(uart, autocomplete, history);
    write!(shell, "Welcome to WindSock{}{}", CR, SHELL_PROMPT).ok();

    interrupt::free(|_| unsafe {
        ENV.replace(Env {
            shell,
            sys,
            led_on: false,
        });
        windsock_hal::arch::register::mstatus::set_mie();
        windsock_hal::arch::register::mie::set_mext();
    });

    loop {
        pmod_led_0.toggle().ok();
        pmod_led_1.toggle().ok();
        pmod_led_2.toggle().ok();
        pmod_led_3.toggle().ok();

        if pmod_btn_1.is_high().unwrap() {
            panic!("Hello panic");
        }

        if pmod_btn_2.is_high().unwrap() {
            timer.delay(100.ms());
        } else {
            timer.delay(200.ms());
        }
    }
}

#[export_name = "MachineExternal"]
fn uart_interrupt() {
    interrupt::free(|_| unsafe {
        ENV.as_mut().map(poll_serial);
    });
}

fn poll_serial(env: &mut Env) {
    loop {
        match env.shell.poll() {
            Err(ShellError::WouldBlock) => break,
            Ok(Some(Input::Command((cmd, _)))) => {
                match cmd {
                    "help" => {
                        env.shell.write_str(HELP).ok();
                    }
                    "clear" => {
                        env.shell.clear().ok();
                    }
                    "on" => {
                        env.sys.set_led_color(0, RgbColor::from_hex(0xFFA500));
                        env.led_on = true;
                        env.shell.write_str(CR).ok();
                    }
                    "off" => {
                        env.sys.set_led_color(0, RgbColor::from_hex(0));
                        env.led_on = false;
                        env.shell.write_str(CR).ok();
                    }
                    "status" => {
                        let status = if env.led_on { "On" } else { "Off" };
                        write!(env.shell, "{0:}Led status: {1:}{0:}", CR, status,).ok();
                    }
                    "" => {
                        env.shell.write_str(CR).ok();
                    }
                    _ => {
                        write!(env.shell, "{0:}unsupported command{0:}", CR).ok();
                    }
                }
                env.shell.write_str(SHELL_PROMPT).ok();
            }
            _ => {}
        }
    }
}
