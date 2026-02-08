/* Copyright Douglas Reis
Licensed under the Apache License, Version 2.0, see LICENSE for details.
SPDX-License-Identifier: Apache-2.0
*/
/* Auto-generated: `tools/rdlgenerator.py gen-linker-script build/rdl.json lib/linker/memory.ld` */
MEMORY
{
    ROM       : ORIGIN = 0x00008000, LENGTH = 0x008000 /*  32.0 KiB */
    SRAM      : ORIGIN = 0x10000000, LENGTH = 0x020000 /* 128.0 KiB */
    FLASH     : ORIGIN = 0x20000000, LENGTH = 0x100000 /*1024.0 KiB */
    RET_SRAM  : ORIGIN = 0x40600000, LENGTH = 0x001000 /*   4.0 KiB */
}