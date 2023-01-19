// not linking the std lib
#![no_std]

// Overwriting all Rust-level Entry Points
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

// telling the compiler not to mangle the function name
// mangling or decorating is a tecnique used in compiler
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

    vga_buffer::print_something();

    // let vga_buffer = 0xb8000 as *mut u8;

    // for (i, &byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xc;
    //     }
    // }

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


