# `loadavg` utility

I wrote this quick utility to solve my frustrations with getting the load
averages into my [tmux] status line. I like to share my `.tmux.conf` between
machines, and `/proc/loadavg` is only available on linux. Also, differences
between the output of `uptime` made parsing the load averages out of there
fragile and annoying.

I also wanted to learn how to make FFI calls to the C standard library in Rust.
`getloadavgs()` was a useful choice for doing so.

## Building and installing

Install Rust and clone this repository. Inside this directory, type `cargo
build --release` and the binary will be in `./target/release/`. Isn't Cargo
great?
