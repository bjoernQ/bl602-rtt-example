# BL602 RTT Example

On the host side you have to use [blash](https://github.com/bjoernQ/blash) with the `rtt` option. (At the time of writing it's on a branch, not on `main` - please use the branch `backtrace`)

A simple `cargo run` should flash the binary via `blash` and show the output.

![BL602 RTT](docs/blash-rtt.gif)

To see full backtraces you should compile and run like this: `cargo run -Z build-std=core --target riscv32imc-unknown-none-elf`

Uncomment the relevant parts in `main.rs` to make the code panic.
