{
  stdenv,
  fetchzip,
  autoPatchelfHook,
  openssl,
  libusb1,
  libgcc,
  libftdi1,
  makeWrapper,
}:  
stdenv.mkDerivation rec {
  pname = "earlgrey_verilator";
  version = "1.0.0_artefacts_r01";

  src = fetchzip {
    url = "https://github.com/engdoreis/opentitan/releases/download/earlgrey_1.0.0_artefacts_r01/sim_verilator.tar.gz";
    sha256 = "sha256-ZJMThEBz9li6eoQd9KRfdXxc1bVuFpHaEHowyPTOOPg=";
  };

  dontConfigure = true;
  dontBuild = true;

  nativeBuildInputs = [
    autoPatchelfHook
    makeWrapper
  ];

  buildInputs = [
    openssl
    libusb1
    libgcc
    libftdi1
  ];

  installPhase = ''
    mkdir -p $out/bin $out/share
    mv ./*.vmem $out/share/
    cp ./* $out/bin/

    wrapProgram $out/bin/Vchip_sim_tb \
      --add-flags "--meminit=rom,$out/share/test_rom_sim_verilator.39.scr.vmem" \
      --add-flags "--meminit=otp,$out/share/img_rma.24.vmem" \
      --run '
        if [ "$#" -gt 0 ]; then
          last_arg="''${@: -1}"
          set -- "''${@:1:($#-1)}" "--meminit=flash0,$last_arg"
        fi
      '
  '';
}
