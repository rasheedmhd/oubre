// not linking the std lib
#![no_std]
// rust has a test framework that it provides by default but the framework, it built into the std lib
// depending on the test crate
// since we are not linking the std lib, we need to spin up our own custom test framework
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

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

//     println!(
//         "
// @boot 
// New Boot, Works on x86_64 arch machines
// : rl(0,0)rV0.00.01 Oubre OS
// Display size: 80 * 25
// mem = 2222.1567
// # login root
// Password: *******
// TopRank Maverick Systems v0.00.01
// --------------------------------

// You have mail [+1]>

// Hello World!
//         "
//     );

    println!("Hello World{}", "!");

    //panic!("{}", "Roses are red, error occured at '{' ;)");

    // use core::fmt::Write;
    // vga_buffer::PRINTER.lock().write_str("Hello again").unwrap();
    // write!(vga_buffer::PRINTER.lock(), ", some numbers: {} {}", 42, 222.1567).unwrap();

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
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

