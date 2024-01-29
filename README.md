# poll-button
Bart Massey 2023

This Rust app for the BBC micro:bit v2 randomizes the LED
display while button A is held down.

## Build and Run

You can follow the instructions from the embedded micro:bit
[*Discovery Book*](https://docs.rust-embedded.org/discovery/microbit/index.html)
to set up your build environment.  Then you can say

    cargo embed --release

to flash and run this.

You can also follow the setup instructions in the `README`
on the `microbit` crate
[repo](https://github.com/nrf-rs/microbit). You can then say

    cargo run --release

By default, this program uses PCG64 as its pseudo-random
number generator. To instead use WyRand:

    cargo run --release --no-default-features --features=wyrand

## License

This work is made available under the "MIT License". Please
see the file `LICENSE.txt` in this distribution for license
terms.
