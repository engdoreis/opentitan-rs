OUTPUT_ARCH(riscv)

INCLUDE memory.ld

_stack_len     = 0x1000;
_heap_len      = 0x0;

MEMORY
{
    stack       : ORIGIN = ORIGIN(SRAM) + LENGTH(SRAM) - _stack_len, LENGTH = _stack_len
    heap        : ORIGIN = ORIGIN(stack) - _heap_len, LENGTH = _heap_len
    ram         : ORIGIN = ORIGIN(SRAM), LENGTH = LENGTH(SRAM) - _stack_len - _heap_len 
    flash       : ORIGIN = ORIGIN(FLASH), LENGTH = LENGTH(FLASH) 
}
_min_stack     = 0x1000;   /* 4KiB - minimum stack space to reserve */
_stack_start   = ORIGIN(stack) + LENGTH(stack);

_entry_point = _vectors_start + 0x80;
_manifest_entry_point = _entry_point - ORIGIN(flash) ;
ENTRY(_entry_point)

/* Rust dependency */
PROVIDE(_stext = ORIGIN(ram) + 0x80);
PROVIDE(_stack_start = ORIGIN(stack) + LENGTH(stack));
PROVIDE(_max_hart_id = 0);
PROVIDE(_hart_stack_size = LENGTH(stack));
PROVIDE(_heap_size = LENGTH(heap));
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
    } > flash

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
    }  > flash

    .rodata : {
        . = ALIGN(4);
        /* Small RO data before large RO data */
        *(.srodata)
        *(.srodata.*)
        *(.rodata);
        *(.rodata.*)
        . = ALIGN(4);
    } > flash

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
    } > ram

    .bss :
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
    } > ram


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
    } > heap

    /* ensure there is enough room for stack */
    .stack (NOLOAD): {
        . = ALIGN(4);
        . = . + _min_stack ;
        . = ALIGN(4);
        stack = . ;
        _stack = . ;
    } > stack
}

