#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(oubre_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use oubre_os::{
        gdt, 
        interrupts, 
        print, 
        println
    };

use x86_64::instructions::interrupts as hardware_interrupts;

// extern "C" tells the compiler to use the C calling convention.
// https://en.wikipedia.org/wiki/Calling_convention
// https://www.rasheedstarlet.com/articles/calling.html
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("
Hi, I am Oubre OS

Works on x86_64 arch machines
version 0.0.1
Display size: 80 * 25
version: v0.00.01


If you are not failing a lot, you are probably not being as creative as you could be - you aren't stretching your imagination - John Backus, Creator of FORTRAN


----------------------------------
Creator: Rasheed Starlet Maverick
Copy Left @ www.starletcapital.com
");
println!("Some test string that fits on a single line");

    init_descriptor_tables();
    init_PICs();

    fn init_descriptor_tables() {
        gdt::init();
        interrupts::init_idt();
    }

    fn init_PICs() {
        unsafe {
            interrupts::PICS.lock().initialize();

            // executes the sti(set interrupt) instruction to enable external interrupts
            // enabling this enables the hardware timer (intel 8253) by default then we start getting
            // timer interrupts which leads to a double fault
            // we need to handle the hardware timer interrupts
            hardware_interrupts::enable();
        }
    }

    #[cfg(test)]
    test_main();

    oubre_os::hlt_loop();
    
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    oubre_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    oubre_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    oubre_os::serial_print!("trivial assertion... ");
    assert_eq!(1, 1);
    oubre_os::serial_println!("[ok]");
}
