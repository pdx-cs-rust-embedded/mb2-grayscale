//! Display demo.  
//! Bart Massey 2024-01
//! Based on the documentation for `microbit::display::nonblocking`.

#![no_std]
#![no_main]

use core::cell::RefCell;

use cortex_m_rt::entry;
use cortex_m::interrupt::Mutex;
#[rustfmt::skip]
use microbit::{
    board::Board,
    display::nonblocking::{Display, GreyscaleImage},
    hal::{
        prelude::*,
        
        pac::{TIMER1, interrupt},
        timer::Timer,
    },
};

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

static DISPLAY: Mutex<RefCell<Option<Display<TIMER1>>>> =
    Mutex::new(RefCell::new(None));

fn handle_display<F>(action: &'static str, mut f: F)
    where F: FnMut(&mut Display<TIMER1>)
{
    cortex_m::interrupt::free(|cs| {
        let mut display_ref = DISPLAY.borrow(cs).borrow_mut();
        match *display_ref {
            Some(ref mut display) => {
                f(display)
            }
            None => panic!("{}", action),
        }
    });
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let display = Display::new(board.TIMER1, board.display_pins);
    cortex_m::interrupt::free(|cs| {
        let mut cell = DISPLAY.borrow(cs).borrow_mut();
        *cell = Some(display);
    });
    let mut timer2 = Timer::new(board.TIMER0);

    rprintln!("starting");
    loop {
        rprintln!("show");
        handle_display("show", |display| {
            display.show(&GreyscaleImage::new(&[
                [0, 5, 0, 5, 0],
                [7, 0, 7, 0, 7],
                [7, 0, 0, 0, 7],
                [0, 7, 0, 7, 0],
                [0, 0, 7, 0, 0],
            ]));
        });
        timer2.delay_ms(1000u32);

        rprintln!("clear");
        handle_display("clear", |display| display.clear());
        timer2.delay_ms(1000u32);
    }
}

#[interrupt]
fn TIMER1() {
    rprintln!("display");
    handle_display("interrupt", |display| display.handle_display_event());
}
