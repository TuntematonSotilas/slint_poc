# slint_poc
POC of Slint

## Doc 
https://slint.dev/releases/1.3.2/docs/tutorial/rust/getting_started


## Install / check required tools
Make sure you have basic tools installed:

- [Rust](https://www.rust-lang.org)
- [cargo-make](https://sagiegurari.github.io/cargo-make/)

Add WASM Target : `rustup target add wasm32-unknown-unknown`

## Run

1. Open a new terminal and run: `cargo make serve`
1. Open a second terminal run: `cargo make watch`

## Lint

Run `cargo make verify` in your terminal to format and lint the code.

## Build for release

`cargo make build_release`