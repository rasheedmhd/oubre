// not linking the std lib
#![no_std]

// Overwriting all Rust-level Entry Points
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(oubre_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use oubre_os::println;

// mod vga_buffer;
// mod serial;

// the Success and Failed codes can  be any arbitrary numbers
// as long as they aren't already used by QeMu
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// #[repr(u32)]
// pub enum QemuExitCode {
//     Success = 0x10,
//     Failed = 0x11,
// }

// pub fn exit_qemu(exit_code: QemuExitCode) {
//     use x86_64::instructions::port::Port;

//     unsafe {
//         let mut port = Port::new(0xf4);
//         port.write(exit_code as u32);
//     }
// }
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

    println!(
        "
@boot<;> Hello World! 


New Boot, Works on x86_64 arch machines
: rl(0,0)rV0.00.01 Oubre OS
Display size: 80 * 25
mem = 2222.1567
# login root
Password: *******
TopRank Maverick Systems v0.00.01
--------------------------------

You have mail [+1]>
        "
    );

 //   println!("Hello World{}", "!");

    //panic!("{}", "Roses are red, error occurred at '{' ;)");

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

    // calling our test entry point
    // annotating it to run in only test contexts
    // #[cfg(test)]
    // test_main();

    loop {}
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


// #[cfg(test)]
// #[panic_handler]
// fn panic(info: &PanicInfo) -> ! {
//     serial_println!("[failed]\n");
//     serial_println!("Error: {}\n", info);
//     exit_qemu(QemuExitCode::Failed);
//     loop {}
// }

// #[cfg(test)]
// fn test_runner(tests: &[&dyn Testable]) {
//     serial_println!("Running {} tests", tests.len());
//     //[...]
//     // println!("Running {} tests", tests.len());
//     for test in tests {
//         test.run();
//     }
//     exit_qemu(QemuExitCode::Success);
// }

#[test_case]
fn trivial_assertion() {
    // serial_print!("trivial assertion... ");
    assert_eq!(1, 1);
    // serial_println!("[ok]");

    // when a test runs and encounters a problem where the test is not able to
    // exit, the bootimage has a 5 mins time for it after which it will exit by force as failed. We can change that time in Cargo.toml
    // loop {}
}

// pub trait Testable {
//     fn run(&self) -> ();
// }

// // implement a Testable trait for an type that can be called like a function
// impl<T> Testable for T 
// where 
//     T: Fn(),
// {
//     fn run(&self) {
//         // prints a string slice of the name of the type / test function
//         serial_print!("{}...\t", core::any::type_name::<T>());
//         // runs the test function
//         self();
//         serial_println!("[ok]");
//     }
// }


