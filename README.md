# tui-uefi

[![CI workflow badge](https://github.com/reubeno/tui-uefi/actions/workflows/ci.yaml/badge.svg)](https://github.com/reubeno/tui-uefi/actions/workflows/ci.yaml)

Provides crates useful for building TUIs in a UEFI application or loader:

* `ratatui-uefi`: implements an output backend usable with [ratatui](https://github.com/ratatui/ratatui)
* `terminput-uefi`: implements an input backend usable with [terminput](https://github.com/aschey/terminput) 

![screenshot](https://github.com/user-attachments/assets/29a559ff-f2c3-4059-8725-95602fdcba63)


## Build

Currently requires building using nightly:

```console
$ cargo +nightly build --target=x86_64-unknown-uefi
```

Only tested with x86_64 but may work just fine for aarch64 as well.

## Running an example

You can use [uefi-run](https://github.com/Richard-W/uefi-run) to test the example, e.g.:

```console
$ cargo install uefi-run
$ cargo +nightly build --target=x86_64-unknown-uefi
$ uefi-run -d /home/reubeno/src/tui-uefi/target/x86_64-unknown-uefi/debug/basic
