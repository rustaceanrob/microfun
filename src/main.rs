#![no_std]
#![no_main]
use cortex_m::asm::nop;
use chacha20::ChaCha20;
use cortex_m_rt::entry;
use panic_halt as _;
use microbit::{
    display::blocking::Display, hal::{prelude::*, uarte::{self, Baudrate, Parity}, Timer}, Board
};
use rtt_target::{rprintln, rtt_init_print};
const DANCE_PROB: u8 = 224;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let nonce = b"Hello, world";
    let key = [0; 32];
    let mut chacha = ChaCha20::new(key, *nonce, 0);
    let mut to = *b"Lite dance";
    rprintln!("!");
    loop {
        let mut light_none = [
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
        ];
        for row in light_none.iter_mut() {
            chacha.apply_keystream(&mut to);
            for (i, t) in to.iter().enumerate() {
                if *t > DANCE_PROB {
                    row[i % 5] = 1;
                }
            }
        }
        display.show(&mut timer, light_none, 100);
        display.clear();
        timer.delay_ms(100_u32);
    }
}
