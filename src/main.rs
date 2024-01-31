//! Display demo.  Based on the documentation for
//! `microbit::display::nonblocking`.
// Bart Massey 2024-01

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use microbit::{
    board::Board,
    display::nonblocking::{Display, GreyscaleImage},
    hal::{
        pac::{self, interrupt, TIMER1},
        prelude::*,
        timer::Timer,
    },
};

use panic_rtt_target as _;
use rtt_target::rtt_init_print;

use mb2_grayscale::LockMut;

/// The display is shared by the main program and the
/// interrupt handler.
static DISPLAY: LockMut<Display<TIMER1>> = LockMut::new();

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let mut board = Board::take().unwrap();
    let display = Display::new(board.TIMER1, board.display_pins);
    DISPLAY.init(display);
    let mut timer2 = Timer::new(board.TIMER0);
    unsafe {
        board.NVIC.set_priority(pac::interrupt::TIMER1, 128);
        pac::NVIC::unmask(pac::interrupt::TIMER1);
    }

    let image = GreyscaleImage::new(&[
        [0, 4, 0, 4, 0],
        [7, 3, 7, 3, 7],
        [7, 2, 2, 2, 7],
        [0, 7, 2, 7, 0],
        [0, 0, 7, 0, 0],
    ]);

    loop {
        DISPLAY.with_lock(|display| display.show(&image));
        timer2.delay_ms(1000u32);

        DISPLAY.with_lock(|display| display.clear());
        timer2.delay_ms(1000u32);
    }
}

#[interrupt]
fn TIMER1() {
    DISPLAY.with_lock(|display| display.handle_display_event());
}
