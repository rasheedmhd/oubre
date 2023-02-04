//static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

// the x86_64 crate has built-in abstractions that allow to create IDT that will map
// CPU exceptions to interrupt handlers
use x86_64::structures::idt::{ InterruptDescriptorTable, InterruptStackFrame };
use crate::println;
use lazy_static::lazy_static;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        // creating an IDT that we can add interrupt handlers to 
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}



// a breakpoint interrupt handler that use the x86-interrupt calling convention 
// that simply prints text and the stack frame 
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

#[test_case]
fn test_breakpoint_exception() {
    // invoking a breakpoint exception 
    x86_64::instructions::interrupts::int3();
}