# Example: basic

This is a basic example that demonstrates how to use the crates in this repo to implement a simple TUI that can run in a UEFI application.

This app is compiled with `std` (i.e., *not* `no_std`), and requires building with a nightly Rust toolchain. It also uses the `uefi` crate from the `rust-osdev` project.
