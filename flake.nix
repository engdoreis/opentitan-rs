# Copyright (c) Douglas Reis.
# Licensed under the Apache License, Version 2.0, see LICENSE for details.
# SPDX-License-Identifier: Apache-2.0
{
  description = "Opentitan-rs environment.";
  inputs = {
    lowrisc-nix.url = "github:lowRISC/lowrisc-nix";

    nixpkgs.follows = "lowrisc-nix/nixpkgs";
    flake-utils.follows = "lowrisc-nix/flake-utils";
    uv2nix = {
      url = "github:pyproject-nix/uv2nix";
      inputs.pyproject-nix.follows = "pyproject-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    pyproject-nix = {
      url = "github:pyproject-nix/pyproject.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    pyproject-build-systems = {
      url = "github:pyproject-nix/build-system-pkgs";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.pyproject-nix.follows = "pyproject-nix";
      inputs.uv2nix.follows = "uv2nix";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
    };
    zermio = {
      url = "github:engdoreis/zermio";
    };
  };

  nixConfig = {
    extra-substituters = ["https://nix-cache.lowrisc.org/public/"];
    extra-trusted-public-keys = ["nix-cache.lowrisc.org-public-1:O6JLD0yXzaJDPiQW1meVu32JIDViuaPtGDfjlOopU7o="];
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    lowrisc-nix,
    rust-overlay,
    ...
  } @ inputs: let
    system_outputs = system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [rust-overlay.overlays.default];
      };

      rust_toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

      workspace = inputs.uv2nix.lib.workspace.loadWorkspace {workspaceRoot = ./.;};
      overlay = workspace.mkPyprojectOverlay {
        sourcePreference = "wheel";
      };

      pythonSet =
        (pkgs.callPackage inputs.pyproject-nix.build.packages {
          python = pkgs.python310;
        }).overrideScope
        (
          pkgs.lib.composeManyExtensions [
            inputs.pyproject-build-systems.overlays.default
            overlay
            (lowrisc-nix.lib.pyprojectOverrides {inherit pkgs;})
          ]
        );

      pythonEnv = pythonSet.mkVirtualEnv "python-env" workspace.deps.default;

      zermio-cli = inputs.zermio.packages.${system}.default;

      earlgrey_verilator = pkgs.callPackage ./nix/verilator_earlgrey.nix {};
      opentitan = pkgs.callPackage ./nix/opentitan.nix {};

      # Runtime shared libs needed by the externally-built opentitantool/hsmtool
      # (see OPENTITAN_TOOLS_DIR below): as non-Nix-built binaries they have no
      # RPATH into the store, so the dynamic linker needs these on
      # LD_LIBRARY_PATH. Versions checked against `ldd` on the actual binaries:
      # libssl/libcrypto, libusb-1.0, libftdi1, libudev, libstdc++.
      opentitanToolRuntimeLibs = with pkgs; [
        openssl
        libusb1
        libftdi1
        libudev-zero
        stdenv.cc.cc.lib
      ];
    in {
      formatter = pkgs.alejandra;
      devShells = rec {
        default = silicon_es;
        silicon_es = pkgs.mkShell {
          name = "silicon-es";
          nativeBuildInputs = with pkgs; [
            rust_toolchain
            picocom
            openocd
            uv
            reuse
            pythonEnv
            zermio-cli
            cargo-binutils
            llvmPackages_20.bintools
            libudev-zero
            libftdi1
          ];

          buildInputs = with pkgs; [libelf zlib] ++ opentitanToolRuntimeLibs;

          # opentitantool/hsmtool are open-source lowRISC/opentitan Rust binaries
          # (//sw/host/opentitantool, //sw/host/hsmtool) built via that repo's own
          # Bazel setup, not something this flake vendors or rebuilds. Build them
          # from a lowRISC/opentitan checkout (./bazelisk.sh build
          # //sw/host/opentitantool //sw/host/hsmtool), symlink/copy both binaries
          # into one directory, and point OPENTITAN_TOOLS_DIR at it; this shell
          # then picks them up.
          #
          # OPENTITAN_TOOLS_DIR must be exported *before* this shell starts
          # (shellHook only runs once, at shell startup) -- setting it inside an
          # already-running shell won't retroactively update PATH here.
          shellHook = ''
            if [ -n "''${OPENTITAN_TOOLS_DIR:-}" ] && [ -d "$OPENTITAN_TOOLS_DIR" ]; then
              export PATH="$OPENTITAN_TOOLS_DIR:$PATH"
            else
              echo "warning: \$OPENTITAN_TOOLS_DIR not set (or not a directory) -- opentitantool/hsmtool won't be on PATH." >&2
              echo "  Build them from a lowRISC/opentitan checkout: ./bazelisk.sh build //sw/host/opentitantool //sw/host/hsmtool" >&2
              echo "  then export OPENTITAN_TOOLS_DIR=<dir containing both binaries>, before entering this shell" >&2
            fi

            export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath opentitanToolRuntimeLibs}:''${LD_LIBRARY_PATH:-}"
            export HSMTOOL_MODULE="${opentitan.libkmsp11}/lib/libkmsp11.so"
          '';
        };

        opentitan_rs = pkgs.mkShell {
          name = "opentitan-rs";
          nativeBuildInputs = with pkgs; [
            rust_toolchain
            picocom
            openocd
            uv
            reuse
            pythonEnv
            zermio-cli
            earlgrey_verilator
            srecord
            cargo-binutils
            llvmPackages_20.bintools
            libudev-zero
            libftdi1
          ];

          buildInputs = with pkgs; [libelf zlib];
        };
      };
    };
  in
    flake-utils.lib.eachDefaultSystem system_outputs;
}
