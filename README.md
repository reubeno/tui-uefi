# tui-uefi

[![CI workflow badge](https://github.com/reubeno/tui-uefi/actions/workflows/ci.yaml/badge.svg)](https://github.com/reubeno/tui-uefi/actions/workflows/ci.yaml)

Provides crates useful for building TUIs (Terminal User Interfaces) in a [UEFI](https://uefi.org/) application or boot loader:

* `ratatui-uefi`: implements an output backend usable with [ratatui](https://github.com/ratatui/ratatui)
* `terminput-uefi`: implements an input backend usable with [terminput](https://github.com/aschey/terminput) 

![screenshot](https://github.com/user-attachments/assets/29a559ff-f2c3-4059-8725-95602fdcba63)

## Build

Firstly add the UEFI target to your toolchain:

```console
$ rustup target add x86_64-unknown-uefi
```

Then build using nightly:

```console
$ cargo +nightly build --target=x86_64-unknown-uefi
```

Only tested with x86_64 but may work just fine for aarch64 as well.

## Running an example

You can use [uefi-run](https://github.com/Richard-W/uefi-run) to run the examples, e.g.:

```console
$ cargo install uefi-run
$ cargo +nightly build --target=x86_64-unknown-uefi
$ uefi-run -d target/x86_64-unknown-uefi/debug/basic.efi -- -enable-kvm
```

_Note: if your environment doesn't support KVM, you will need to remove the `-enable-kvm` option._
