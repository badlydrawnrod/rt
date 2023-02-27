OUTPUT_FORMAT("elf32-littleriscv", "elf32-littleriscv", "elf32-littleriscv")

OUTPUT_ARCH(riscv)
ENTRY(_start)

/* Configuration variables to be used by the MEMORY command. */
ROM_START = 0x00000000;
ROM_SIZE = 0x4000;  /* 16KiB  */
RAM_SIZE = 0x4000;  /* 16KiB */

/* The stack grows down from the end of RAM. */
STACK_TOP = ROM_START + ROM_SIZE + RAM_SIZE;

/* Memory space definition. These blocks of memory will be used in the SECTIONS command.
 *
 * The robot is a bare-metal machine, i.e., there is no operating system, so its program is loaded into ROM.
 */
MEMORY
{
    rom (rx) :  ORIGIN = ROM_START, LENGTH = ROM_SIZE
    ram (rwx) : ORIGIN = ROM_SIZE,  LENGTH = RAM_SIZE
}

SECTIONS
{
    PROVIDE (__stack_top = STACK_TOP);

    /* Startup code */
    .init ORIGIN(rom) :
    {
        KEEP(*(.init));
        KEEP(*(.init.rust));
    } > rom

    /* Code in ROM. */
    .text :
    {
        *(.text, .text.*)
    } > rom

    /* Read-only data in ROM. */
    .rodata : ALIGN(4)
    {
        *(.rodata .rodata.* .gnu.linkonce.r.*)
        . = ALIGN(4);        
    } > rom

    /* Initialised data in RAM. Note: this expects the loader to do it, otherwise this would separate the LMA and VMA. */
    .data : ALIGN(4)
    {
        _sdata = .;
        /* Must be called __global_pointer$ for linker relaxations to work. */
        PROVIDE(__global_pointer$ = . + 0x800);
        *(.sdata .sdata.* .sdata2 .sdata2.*);
        *(.data .data.*);
        . = ALIGN(4);
        _edata = .;
    } > ram

    /* Uninitialised data in RAM. */
    .bss (NOLOAD) :
    {
        _sbss = .;
        *(.sbss .sbss.* .bss .bss.*);
        . = ALIGN(4);
        _ebss = .;
    } > ram

    /DISCARD/ :
    {
        *(.eh_frame)
    }

    . = ALIGN(32 / 8);
    __BSS_END__ = .;

    _end = .; PROVIDE (end = .);
}