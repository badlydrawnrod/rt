#![no_std]

use core::arch::global_asm;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Startup code assuming that initialised data is in ROM and has to be copied to RAM.
#[cfg(not(feature = "no_copy"))]
global_asm!(
    "
    .section .init

_start:
    // Initialise the global pointer.
    .option push
    .option norelax
    la      gp,__global_pointer$
    .option pop
    
    // Initialise the stack.
    la      sp, __stack_top
    add     s0, sp, zero

    // Clear the BSS segment.
    la      a0, _sbss
    la      a1, _ebss
    li      a2, 0

clear_bss:
    bgeu    a0, a1, finish_bss
    sb      a2, 0(a0)
    addi    a0, a0, 1
    beqz    zero, clear_bss

finish_bss:
    // Copy the initialised data into RAM.
    la      a0, _sdata
    la      a1, _edata
    la      a2, _sidata

copy_data:
    bgeu    a0, a1, finish_copy
    lb      t0, 0(a2)
    sb      t0, 0(a0)
    addi    a0, a0, 1
    addi    a2, a2, 1
    beqz    zero, copy_data

finish_copy:
    // Call main() so that it doesn't need to be a divergent function.
    call    main
    
    // Halt and catch fire.
    ebreak
"
);

// Startup code assuming that initialised data is placed in RAM by a loader.
#[cfg(feature = "no_copy")]
global_asm!(
    "
    .section .init

_start:
    // Initialise the global pointer.
    .option push
    .option norelax
    la      gp,__global_pointer$
    .option pop
    
    // Initialise the stack.
    la      sp, __stack_top
    add     s0, sp, zero

    // Clear the BSS segment.
    la      a0, _sbss
    la      a1, _ebss
    li      a2, 0

clear_bss:
    bgeu    a0, a1, finish_bss
    sb      a2, 0(a0)
    addi    a0, a0, 1
    beqz    zero, clear_bss

finish_bss:
    // Call main() so that it doesn't need to be a divergent function.
    call    main
    
    // Halt and catch fire.
    ebreak
"
);
