{
  stdenv,
  fetchFromGitHub,
  pythonEnv,
  verilator,
  libelf,
  zlib,
}:
stdenv.mkDerivation rec {
  pname = "earlgrey_verilator";
  version = "master";

  src = fetchFromGitHub {
    owner = "lowRISC";
    repo = "opentitan";
    fetchSubmodules = true;
    rev = "a18de4282866f675ae903af8425d5cf7b69fa195";
    sha256 = "sha256-LaPJTFzyurIP+RxpiPgrZfIWD9kBXExzawyD79bjr+M=";
  };

  dontConfigure = true;

  nativeBuildInputs = [
    pythonEnv
    verilator
  ];

  # OpenTitan requires specific Python dependencies for its build system (Mako, etc.)
  buildInputs = [
    libelf
    zlib
  ];

  buildPhase = ''
    export HOME=$TMPDIR
    fusesoc --verbose --cores-root=. \
      run --flag=fileset_top --target=sim --setup \
      --build-root="./hw" \
      --build lowrisc:dv:top_earlgrey_chip_verilator_sim \
      --verilator_options="+define+RVFI -j $NIX_BUILD_CORES" \
      --make_options="-j $NIX_BUILD_CORES"
  '';

  installPhase = ''
    mkdir -p $out/bin
    cp hw/lowrisc_dv_top_earlgrey_chip_verilator_sim_0.1/sim-verilator/Vchip_sim_tb $out/bin/verilator_earlgrey
  '';
}
