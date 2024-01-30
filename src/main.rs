//! Display demo.  
//! Bart Massey 2024-01
//! Based on the documentation for `microbit::display::nonblocking`.

#![no_std]
#![no_main]

use cortex_m_rt::entry;
#[rustfmt::skip]
use microbit::{
    board::Board,
    display::nonblocking::{Display, GreyscaleImage},
    hal::{
        prelude::*,
        
        pac::{self, TIMER1, interrupt},
        timer::Timer,
    },
};

use panic_rtt_target as _;
use rtt_target::rtt_init_print;

use mb2_grayscale::LockMut;

static DISPLAY: LockMut<Display<TIMER1>> = LockMut::empty();

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let mut board = Board::take().unwrap();
    let display = Display::new(board.TIMER1, board.display_pins);
    DISPLAY.set(display);
    let mut timer2 = Timer::new(board.TIMER0);
    unsafe {
        board.NVIC.set_priority(pac::Interrupt::TIMER1, 128);
        pac::NVIC::unmask(pac::Interrupt::TIMER1);
    }

    loop {
        let image = GreyscaleImage::new(&[
            [0, 5, 0, 5, 0],
            [7, 0, 7, 0, 7],
            [7, 0, 0, 0, 7],
            [0, 7, 0, 7, 0],
            [0, 0, 7, 0, 0],
        ]);
        DISPLAY.lock(|display| display.show(&image));
        timer2.delay_ms(1000u32);

        DISPLAY.lock(|display| display.clear());
        timer2.delay_ms(1000u32);
    }
}

#[interrupt]
fn TIMER1() {
    DISPLAY.lock(|display| display.handle_display_event());
}
