#!/usr/bin/env bash
set -e 

ROOT_DIR=$(dirname "$0")/..
BIN=$1.bin
VMEM=$1.vmem
ARTEFACTS=$ROOT_DIR/artefacts/verilator.tar.gz
TARGET_DIR=$(dirname "$1")

TEST_ROM=$TARGET_DIR/test_rom.39.scr.vmem
OTP=$TARGET_DIR/img_rma.24.vmem

if [ ! -d "$TEST_ROM" ]; then
    echo "Unpacking $ARTEFACTS..."
    pushd $TARGET_DIR
    tar -xzvf "$ARTEFACTS" --strip-components=1
    popd
fi

llvm-objdump --disassemble --demangle --source $1 > $1.asm
llvm-objcopy -O binary $1 $BIN

srec_cat $BIN --binary --offset 0x0 --byte-swap 8 --fill 0xff \
  -within $BIN  -binary -range-pad 8 --output $VMEM --vmem 64

verilator_earlgrey \
      --meminit=flash0,$VMEM \
      --meminit=rom0,$TEST_ROM \
      --meminit=otp,$OTP

