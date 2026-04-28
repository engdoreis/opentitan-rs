/* Copyright Douglas Reis
Licensed under the Apache License, Version 2.0, see LICENSE for details.
SPDX-License-Identifier: Apache-2.0
*/

OUTPUT_ARCH(riscv)

INCLUDE memory.ld

_stack_len     = 0x1000;
_heap_len      = 0x0;
__romext_size_ = 0x10000;

MEMORY
{
    volatile_mem     : ORIGIN = ORIGIN(SRAM), LENGTH = LENGTH(SRAM) - _stack_len - _heap_len 
    non_volatile_mem : ORIGIN = ORIGIN(FLASH) + __romext_size_, LENGTH = LENGTH(FLASH) - __romext_size_
}

_entry_point = _vectors_start + 0x80;

_manifest_entry_point = _entry_point - ORIGIN(non_volatile_mem) ;
_manifest_code_start = _vectors_start - ORIGIN(non_volatile_mem);
_manifest_code_end = __eidata - ORIGIN(non_volatile_mem);

ENTRY(_entry_point)

/* Rust dependency */
PROVIDE(_stext = ORIGIN(volatile_mem) + 0x80);
PROVIDE(_stack_start = __stack_start);
PROVIDE(_max_hart_id = 0);
PROVIDE(_hart_stack_size = _stack_len);
PROVIDE(__stack_size_ = _stack_len);
PROVIDE(_heap_size = _heap_len);
PROVIDE(__pre_init = default_pre_init);

PROVIDE(_default_mp_hook = abort);
PROVIDE(_mp_hook = _default_mp_hook);

EXTERN(_default_start_trap);
PROVIDE(_start_trap = _default_start_trap);

EXTERN(_default_setup_interrupts);
PROVIDE(_setup_interrupts = _default_setup_interrupts);

EXTERN(_default_abort);
PROVIDE(abort = _default_abort);
PROVIDE(ExceptionHandler = abort);
PROVIDE(DefaultHandler = abort);
PROVIDE(InstructionMisaligned = ExceptionHandler);
PROVIDE(InstructionFault = ExceptionHandler);
PROVIDE(IllegalInstruction = ExceptionHandler);
PROVIDE(Breakpoint = ExceptionHandler);
PROVIDE(LoadMisaligned = ExceptionHandler);
PROVIDE(LoadFault = ExceptionHandler);
PROVIDE(StoreMisaligned = ExceptionHandler);
PROVIDE(StoreFault = ExceptionHandler);
PROVIDE(UserEnvCall = ExceptionHandler);
PROVIDE(SupervisorEnvCall = ExceptionHandler);
PROVIDE(MachineEnvCall = ExceptionHandler);
PROVIDE(InstructionPageFault = ExceptionHandler);
PROVIDE(LoadPageFault = ExceptionHandler);
PROVIDE(StorePageFault = ExceptionHandler);

PROVIDE(SupervisorSoft = DefaultHandler);
PROVIDE(MachineSoft = DefaultHandler);
PROVIDE(SupervisorTimer = DefaultHandler);
PROVIDE(MachineTimer = DefaultHandler);
PROVIDE(SupervisorExternal = DefaultHandler);
PROVIDE(MachineExternal = DefaultHandler);

PROVIDE(_start_SupervisorSoft_trap = _start_DefaultHandler_trap);
PROVIDE(_start_MachineSoft_trap = _start_DefaultHandler_trap);
PROVIDE(_start_SupervisorTimer_trap = _start_DefaultHandler_trap);
PROVIDE(_start_MachineTimer_trap = _start_DefaultHandler_trap);
PROVIDE(_start_SupervisorExternal_trap = _start_DefaultHandler_trap);
PROVIDE(_start_MachineExternal_trap = _start_DefaultHandler_trap);

PROVIDE(_pre_init_trap = _default_abort);

PROVIDE(hal_main = main);

SECTIONS
{
    .manifest : ALIGN(4) {
      KEEP(*(.manifest))
    } > non_volatile_mem

    .text : {
        . = ALIGN(4);
    _vectors_start = .;
        KEEP(*(.vectors))
    _vectors_end = .;
        . += 0x80;
        /* Rust dependency */
        KEEP(*(.init));
        KEEP(*(.init.rust));
        KEEP(*(.trap.vector));
        KEEP(*(.trap.start));
        KEEP(*(.trap.start.*));
        KEEP(*(.trap.continue));
        KEEP(*(.trap.rust));   
        KEEP(*(.trap .trap.*));

        *(.text.abort);
        *(.text)
        *(.text.*)
        . = ALIGN(4);
    }  > non_volatile_mem

    .rodata : {
        . = ALIGN(4);
        /* Small RO data before large RO data */
        *(.srodata)
        *(.srodata.*)
        *(.rodata);
        *(.rodata.*)
        . = ALIGN(4);
    } > non_volatile_mem

    .data : {
        . = ALIGN(4);
        /* _sidata = LOADADDR(.data); */
        __sidata = LOADADDR(.data);
        /* _sdata = .; */
        __sdata = .;

        /* Small data before large data */
        *(.sdata)
        *(.sdata.*)
        *(.data);
        *(.data.*)
        . = ALIGN(4);
        /* _edata = .; */
        __edata = .;
        __eidata = LOADADDR(.data);
    } > volatile_mem AT> non_volatile_mem

    .bss (NOLOAD):
    {
        . = ALIGN(4);
        _bss_start = .;
        /* Small BSS before large BSS */
        *(.sbss)
        *(.sbss.*)
        *(.bss)
        *(.bss.*)
        *(COMMON)
        _bss_end = .;
        . = ALIGN(4);
    } > volatile_mem


    /* Rust dependency */
    /* _sbss = _bss_start; */
    __sbss = _bss_start;
    /* _ebss = _bss_end; */
    __ebss = _bss_end;

    .heap (NOLOAD):
    {
        . = ALIGN(4);
        __heap_start = .;
        . = . + _heap_size ;
        __heap_end = .;
        . = ALIGN(4);
    } > volatile_mem

    /* ensure there is enough room for stack */
    .stack (NOLOAD): {
        . = ALIGN(4);
        _stack_end = .;
        . = . + __stack_size_ ;
        . = ALIGN(4);
        stack = . ;
        _stack = . ;
        __stack_start = .;
    } > volatile_mem
}

