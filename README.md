# rt

A very basic RISC-V runtime for use with Arviss. Link this with your binaries. See [rt_app](https://github.com/badlydrawnrod/rt_app) for an example.

# Building

- Install the prerequisites (see below) if you haven't already

- Perform a release build

```
$ cargo build --release
```

This won't actually do much other than to prove that you have everything installed. In reality you'd add this crate `rt` when compiling your binary.

# Prerequisites

You'll need to install:
- Rust itself
- the `riscv32i-unknown-elf` target
- cargo-binutils and llvm-tools

## Install Rust

- Install Rust by following the directions [here](https://www.rust-lang.org/tools/install)

You can use whichever editor you like, although you probably won't go wrong if you install VSCode with the `rust-analyzer` extension.

## Install the `riscv32i-unknown-elf` target

You'll need to install the `riscv32i-unknown-elf` target so that you can build for RISC-V. There are other RISC-V targets, but at the time of writing Arviss supports `rv32imf`, so `riscv32i` is an appropriate choice.

- Install the `riscv32i-unknown-elf` target

```
$ rustup target add riscv32i-unknown-elf
```

- Check that it is installed

```
$ rustup show
Default host: x86_64-unknown-linux-gnu
rustup home:  /home/rod/.rustup

installed targets for active toolchain
--------------------------------------

riscv32i-unknown-none-elf
x86_64-unknown-linux-gnu

active toolchain
----------------

stable-x86_64-unknown-linux-gnu (default)
rustc 1.66.1 (90743e729 2023-01-10)
```

## Install `cargo-binutils` and `llvm-tools`

Together, `cargo-binutils` and `llvm-tool` allow you to invoke LLVM tools such as `llvm-objdump`, `llvm-objcopy`, etc, via `cargo`. What this means in practice is that the tools are integrated with `cargo` so `cargo objdump` will build your code then run `llvm-objdump` on the result.

- Install `cargo-binutils`

```
$ cargo install cargo-binutils
```

- Install `llvm-tools`

```
$ rustup component add llvm-tools
```
