#!/usr/bin/env bash

ELF=$1
BIN=$ELF.bin
PRESIGNING=$ELF.pre-signing
DIGEST=$ELF.digest
SIG=$ELF.ecdsa-sig
SIGNED_BIN=$ELF.signed.bin
IMG=$ELF.img

# Set by user before activating the nix shell.
RELEASE_DIR=$OPENTITAN_TOOLS_DIR
ROM_EXT_BIN=$RELEASE_DIR/proda/rom_ext_real_prod_signed_slot_a_silicon_creator.signed.bin

: "${HSMTOOL_MODULE:?HSMTOOL_MODULE not set -- enter the nix devshell (nix develop) first}"

set -e

llvm-objdump --disassemble --demangle $1 > $ELF.dump &
llvm-objcopy -O binary $ELF $BIN

opentitantool  '--rcfile='  image  manifest  update \
--manifest=$RELEASE_DIR/manifest.json \
    --output=$PRESIGNING $BIN --rsa-key=$RELEASE_DIR/proda/earlgrey_z1_proda_1.der

opentitantool  '--rcfile=' --quiet image digest  --bin=$DIGEST  $PRESIGNING

echo "Signing with proda key..."
KMS_PKCS11_CONFIG=$RELEASE_DIR/proda/earlgrey_z1_proda.yaml \
  hsmtool  '--lockfile=/tmp/hsmtool.lock' --quiet  '--profile=earlgrey_z1_proda' rsa sign \
  --little-endian  '--format=sha256-hash'  '--label=earlgrey_z1_proda_1' --output=$SIG  $DIGEST

opentitantool  '--rcfile='   image  manifest  update '--update-length=false' \
    --output=$SIGNED_BIN $PRESIGNING --rsa-signature=$SIG

opentitantool '--rcfile=' image assemble '--mirror=false' --output=$IMG \
  $ROM_EXT_BIN@0  $SIGNED_BIN@0x10000

echo generated $IMG

echo "Loading..."
opentitantool --logging=info --interface=ftdi --exec="transport init" \
      --exec="bootstrap --clear-uart=true $IMG" \
      --exec="console --non-interactive --exit-success='PASS.*\n' --exit-failure='((FAIL|FAULT).*\n)|(BFV:[0-9a-f]{8})'" \
      no-op


