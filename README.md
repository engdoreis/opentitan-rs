# OpenTitan-rs 

This a hobby project focused on developing Rust embedded drivers for the OpenTitan (Earlgrey) chip. A Verilator model is provided for testing and examples.

## Getting started
Install dependencies with nix:
```sh
nix develop
```

Runing hello world:
```sh
cargo run --bin hello_world --target riscv32imc-unknown-none-elf
```
The uart output can be checked with:
```sh
tail -f uart0.log

```

Running tests with cargo:
```sh
cargo xtask test
```

The documentation can be rendered with:
```sh
cargo doc --open
```

