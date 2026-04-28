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

## Signing artefacts

Running on real Voyager hardware (`tools/run_voyager.sh`) requires signing the ROM_EXT with
`opentitantool` and `hsmtool`, plus a manifest template and the proda signing material (public key,
PKCS#11 token config, signed ROM_EXT binary). These are not sufficient on their own to sign binaries;
see the guide in the official
[documentation](https://opentitan.org/book/signing/index.html?highlight=sign#signing-guide). The
public part of the signing infrastructure is published in the
[earlgrey_es_sival](https://github.com/lowRISC/opentitan/tree/earlgrey_es_sival/sw/device/silicon_creator/rom_ext/proda)
branch of OpenTitan. To sign binaries you need an active opentitan.org account to use the Nitrokeys.

None of this is vendored in this repo. Instead, point `OPENTITAN_TOOLS_DIR` (set *before* running
`nix develop` — the devshell only picks it up at startup) at a local directory containing:

- `opentitantool`, `hsmtool` — built from a lowRISC/opentitan checkout
  (`./bazelisk.sh build //sw/host/opentitantool //sw/host/hsmtool`).
- `manifest.json` — the ROM_EXT manifest template used when signing.
- `proda/` — production signing material (`earlgrey_z1_proda`).
- `sival/` — SIVAL signing material (`earlgrey_z1_sival`).

The devshell builds `libkmsp11` (the Google Cloud KMS PKCS#11 provider used by `hsmtool`) itself and
exports it as `HSMTOOL_MODULE`.

