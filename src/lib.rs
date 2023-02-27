#![no_std]

use core::arch::global_asm;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

global_asm!(
    "   .section .init",
    // "   .global _start",
    // "   .type   _start, @function",
    "_start:",
    // "   .cfi_startproc",
    // "   .cfi_undefined ra",
    //  Initialise the global pointer.
    "   .option push",
    "   .option norelax",
    "   la      gp,__global_pointer$",
    "   .option pop",
    //  Initialise the stack.
    "   la      sp, __stack_top",
    "   add     s0, sp, zero",
    //  Clear the BSS segment.
    "   la      a0, _sbss",
    "   la      a1, _ebss",
    "   li      a2, 0",
    "clear_bss:",
    "   bgeu    a0, a1, finish_bss",
    "   sb      a2, 0(a0)",
    "   addi    a0, a0, 1",
    "   beqz    zero, clear_bss",
    "finish_bss:",
    //  Call main() so that it doesn't need to be a divergent function.
    "   call        main",
    //  Halt and catch fire.
    "   ebreak",
    // "   .cfi_endproc",
);
