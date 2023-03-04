#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

// the x86-interrupt calling convention is an unstable feature we need to mark it as such 
#![feature(abi_x86_interrupt)]

// rust has a test framework that it provides by default but the framework, it built into the std lib
// depending on the test crate
// since we are not linking the std lib, we need to spin up our own custom test framework

// THE CUSTOM TEST FRAMEWORKS
//     Generates a main func that calls the test_runner
//     But this is ignored bc of #![no_main] 
//     so we need to define an entry point
//     which we can call in _start
// #![feature(custom_test_frameworks)]
// #![test_runner(crate::test_runner)]
// //  test_runner entry point 
// #![reexport_test_harness_main = "test_main"]

pub mod vga_buffer;
pub mod serial;
pub mod interrupts;
pub mod gdt;

use core::panic::PanicInfo;

// the Success and Failed codes can  be any arbitrary numbers
// as long as they aren't already used by QeMu
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32)
    }
}


pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T 
where 
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

//#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}


// A panic fn that prints to the host OS console using (UART)
// Universal Async Receiver - Transmitter to communicate to it 
pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[Failed]\n");
    serial_println!("[Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[Failed]\n");
    serial_println!("[Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {};
}

pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe {
        interrupts::PICS.lock().initialize();

        // executes the sti(set interrupt) instruction to enable external interrupts 
        // enabling this enables the hardware timer (intel 8253) by default then we start getting
        // timer interrupts which leads to a double fault 
        // we need to handle the hardware timer interrupts 
        x86_64::instructions::interrupts::enable();
    }
}