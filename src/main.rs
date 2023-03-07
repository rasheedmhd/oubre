#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(oubre_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use oubre_os::{ print, println };

// telling the compiler not to mangle the function name
// mangling or decorating is a technique used in compiler
// design to ensure that the compiler has unique names to
// variable bindings, function names etc
//https://en.wikipedia.org/wiki/Name_mangling

//static HELLO: &[u8] = b"Hello World!, I am Oubre, an Operating System created by Starlet ;)";
// extern "C" tells the compiler to use the C calling convention.
// Calling conventions are a low level implementation of how functions
// should receive parameters from calling functions and how to return the results.
// https://en.wikipedia.org/wiki/Calling_convention
#[no_mangle]
pub extern "C" fn _start() -> ! {

println!("
Hi, I am Oubre OS

Works on x86_64 arch machines
version 0.0.1
Display size: 80 * 25

Copy Left @ Starlet Capital 
version: v0.00.01
--------------------------------

creator: Rasheed Starlet Maverick 

If you are not failing a lot, you are probably not being as creative as you could be - you aren't stretching your imagination - John Backus, Creator of FORTRAN

");


    // calling out init function in lib.rs which in turn
    // calls idt_init() in interrupts.rs to load the
    // InterruptDescriptorTable to the CPU
    oubre_os::init();

    // invoking a breakpoint exception where the CPU will responds by
    // running the breakpoint interrupt handler
    // x86_64::instructions::interrupts::int3();

    // triggering a page fault
    // unsafe {
    //     *(0xdeadbeef as *mut u64) = 42;
    // }
    // #[allow(unconditional_recursion)]
    // fn stack_overflow() {
    //     stack_overflow(); // for each recursion the return address is pushed to the stack
    // }

    // // triggering a stack overflow
    // stack_overflow();

    // calling our test entry point
    // annotating it to run in only test contexts
    #[cfg(test)]
    test_main();

    //println!("It did not crash!");

    loop {
        //print!("%%");
    }
}


/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    oubre_os::test_panic_handler(info)
}


#[test_case]
fn trivial_assertion() {
    // serial_print!("trivial assertion... ");
    assert_eq!(1, 1);
    // serial_println!("[ok]");

    // when a test runs and encounters a problem where the test is not able to
    // exit, the bootimage has a 5 mins time for it after which it will exit by force as failed. We can change that time in Cargo.toml
    // loop {}
}




