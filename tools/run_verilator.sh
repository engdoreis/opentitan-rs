#!/usr/bin/env bash
set -e 

ROOT_DIR=$(dirname "$0")/..
BIN=$1.bin
VMEM=$1.vmem

llvm-objcopy -O binary $1 $BIN
srec_cat $BIN --binary --offset 0x0 --byte-swap 8 --fill 0xff \
  -within $BIN  -binary -range-pad 8 --output $VMEM --vmem 64

set +e
Vchip_sim_tb "$VMEM"
exit 0


