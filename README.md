# mb2-grayscale: `microbit-v2` nonblocking `Display` demo
Bart Massey 2024

This code is heavily adapted from crate examples and
documentation in the `microbit-v2` crate. It provides a demo
of using `microbit_v2::display::nonblocking::Display` to
display "grayscale" images.

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

## License

This work is made available under the "MIT License". Please
see the file `LICENSE.txt` in this distribution for license
terms.
