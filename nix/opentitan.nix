# Copyright (c) Douglas Reis.
# Licensed under the Apache License, Version 2.0, see LICENSE for details.
# SPDX-License-Identifier: Apache-2.0
{
  stdenv,
  fetchurl,
  autoPatchelfHook,
  openssl,
}: {
  # Google Cloud KMS PKCS#11 provider, used by hsmtool to talk to Cloud KMS.
  # Fetched the same way OpenTitan's own Bazel build does it
  # (third_party/hsm/extensions.bzl in lowRISC/opentitan): a public, checksummed
  # release tarball, so there is no need to vendor this binary locally.
  libkmsp11 = stdenv.mkDerivation {
    pname = "libkmsp11";
    version = "1.8";

    src = fetchurl {
      url = "https://github.com/GoogleCloudPlatform/kms-integrations/releases/download/pkcs11-v1.8/libkmsp11-1.8-linux-amd64.tar.gz";
      sha256 = "3b932f22a8abb631442c3276e9c309554c81855526b74fbc9edaddcb57a557f7";
    };

    dontConfigure = true;
    dontBuild = true;

    nativeBuildInputs = [autoPatchelfHook];
    buildInputs = [openssl];

    installPhase = ''
      mkdir -p $out/lib
      install -m755 libkmsp11.so $out/lib/
    '';
  };
}
