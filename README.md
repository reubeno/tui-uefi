# tui-uefi

Provides crates useful for building TUIs in a UEFI application or loader:

* `ratatui-uefi`: implements an output backend usable with [ratatui](https://github.com/ratatui/ratatui)
* `terminput-uefi`: implements an input backend usable with [terminput](https://github.com/aschey/terminput) 

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
