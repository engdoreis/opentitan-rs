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
      lrPkgs = lowrisc-nix.outputs.packages.${system};

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
    in {
      formatter = pkgs.alejandra;
      devShells = rec {
        default = opentitan_rs;
        opentitan_rs = pkgs.mkShell {
          name = "opentitan-rs";
          nativeBuildInputs = with pkgs; [
            rust_toolchain
            picocom
            openocd
            uv
            pythonEnv
            zermio-cli
            earlgrey_verilator
            srecord
            cargo-binutils
            llvmPackages_20.bintools
          ];

          buildInputs = with pkgs; [libelf zlib];
        };
      };
    };
  in
    flake-utils.lib.eachDefaultSystem system_outputs;
}
