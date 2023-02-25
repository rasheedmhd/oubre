#![no_std]
#![no_main]

#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use oubre_os::{ exit_qemu, QemuExitCode, serial_print, serial_println };

lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut test_idt = InterruptDescriptorTable::new();
        unsafe {
            test_idt.double_fault
            .set_handler_fn(test_double_fault_handler)
            .set_stack_index(oubre_os::gdt::DOUBLE_FAULT_IST_INDEX);
        }
        test_idt
    };
}

extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: InterruptStackFrame, 
    _error_code: u64,
) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

pub fn init_test_idt() {
    TEST_IDT.load();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_print!("stack_overflow::stack_overflow...\t");

    oubre_os::gdt::init();
    init_test_idt();

    stack_overflow();

    panic!("Execution continued after a stack overflow");
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow(); // for each recursion, the return address is pushed 
    volatile::Volatile::new(0).read(); // prevent tail recursion optimizations 
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    oubre_os::test_panic_handler(info);
}
