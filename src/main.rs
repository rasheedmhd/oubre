// not linking the std lib
#![no_std]

// Overwriting all Rust-level Entry Points
#![no_main]

use core::panic::PanicInfo;

// telling the compiler not to mangle the function name
// mangling or decorating is a tecnique used in compiler
// design to ensure that the compiler has unique names to
// variable bindings, function names etc
//https://en.wikipedia.org/wiki/Name_mangling
#[no_mangle]

// extern "C" tells the compiler to use the C calling convention.
// Calling conventions are a low level implementation of how functions
// should receive parameters from calling functions and how to return the results.
// https://en.wikipedia.org/wiki/Calling_convention
pub extern "C" fn _start() -> ! {
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
