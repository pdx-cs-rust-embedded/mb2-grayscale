//! Display demo.
//!
//! XXX Will fix later.

#![no_std]
#![no_main]

use core::cell::OnceCell;

use cortex_m_rt::{entry, interrupt};
use cortex_m::interrupt::Mutex;
#[rustfmt::skip]
use microbit::{
    board::Board,
    display::nonblocking::{Display, GreyscaleImage},
    hal::{
        prelude::*,
        
        pac::TIMER0,
        timer::Timer,
    },
};

use panic_halt as _;

static DISPLAY: Mutex<OnceCell<Display<TIMER0>>> = Mutex::new(OnceCell::new());

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let display = Display::new(board.TIMER1, board.display_pins);
    let mutex = DISPLAY.lock().unwrap();
    mutex.set(display).unwrap();
    drop(mutex);

    let mut timer2 = Timer::new(board.TIMER0);
    loop {
        let display = DISPLAY.lock().unwrap().get_mut().unwrap();
        display.show(&GreyscaleImage::new(&[
            [0, 5, 0, 5, 0],
            [7, 0, 7, 0, 7],
            [7, 0, 0, 0, 7],
            [0, 7, 0, 7, 0],
            [0, 0, 7, 0, 0],
        ]));
        drop(display);
        timer2.delay_ms(1000u32);

        let display = DISPLAY.lock().unwrap().get_mut().unwrap();
        display.clear();
        drop(display);
        timer2.delay_ms(1000u32);
    }
}

#[interrupt]
fn TIMER0() {
    let display = DISPLAY.lock().unwrap().get_mut().unwrap();
    display.handle_display_event();
}
