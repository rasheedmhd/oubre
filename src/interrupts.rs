use crate::{ 
    gdt,
    println,
    print
};

use x86_64::structures::idt::{ 
    InterruptDescriptorTable, 
    InterruptStackFrame,
    PageFaultErrorCode
};

use x86_64::registers::control::Cr2;
//------------------------------------ keyboard_interrupt_handler imports
use x86_64::instructions::port::Port; 
use pc_keyboard::{
    layouts, 
    DecodedKey, 
    HandleControl, 
    Keyboard,
    ScancodeSet1
};
//------------------------------------ end of keyboard_interrupt_handler

use lazy_static::lazy_static;
use pic8259::ChainedPics;
use spin::Mutex;

pub const PRIMARY_PIC_OFFSET: u8 = 32;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PRIMARY_PIC_OFFSET,
    Keyboard,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}



pub static PICS:  Mutex<ChainedPics> = Mutex::new(
    unsafe {
        ChainedPics::new_contiguous(PRIMARY_PIC_OFFSET)
    }
);

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {

        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);

        // MEMORY MANAGEMENT
        idt.page_fault.set_handler_fn(page_fault_handler);

        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
            .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        //idt[32].set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);
        idt

    };
}

pub fn init_idt() {
    IDT.load();
}

// x86_64 arch does not allow returning from a double fault so
// the exception handler should diverge ( -> !)
// the _error_code is always 0
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame, _error_code: u64) -> !
{
    panic!("EXCEPTION: DOUBLE FAULT \n {:#?}", stack_frame);
}

// a breakpoint interrupt handler that use the x86-interrupt calling convention
// that simply prints text and the stack frame
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
    //oubre_os::hlt_loop();
}

// #[test_case]
// fn test_breakpoint_exception() {
//     // invoking a breakpoint exception
//     x86_64::instructions::interrupts::int3();
// }

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame, 
    error_code: PageFaultErrorCode
) 
{
    println!("EXCEPTION: PAGE FAULT");
    println!("Accessed Address: {:#?}", Cr2::read());
    println!("Error Code: {:?}", error_code);
    println!("Stack frame:\n{:#?}", stack_frame);
    crate::hlt_loop();
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame)
{
    print!(".");

    unsafe {
        PICS.lock()
        .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

static mut caps_lock_state: bool = false;

extern "x86-interrupt" fn keyboard_interrupt_handler(stack_frame: InterruptStackFrame) 
{

    // lazy_static! {
    //     static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = 
    //         Mutex::new(
    //             Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore)
    //         );
    // }

    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = {
            let keyboard = Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore));
            keyboard
        };
    }

    unsafe {
        PICS.lock()
        .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());

        let mut keyboard = KEYBOARD.lock();
        let mut port = Port::new(0x60);
    
        let scancode: u8 = port.read();
    
        let key = match scancode  {
    
            0x1C => Some("\n"),
            0x09 => Some("8"),
            0x1E => Some("A"),
            0x3A => {
                caps_lock_state = !caps_lock_state;
                None
            },
    
            _ => None
        };
    
        if caps_lock_state {
            if let Some(scancode) = key {
                print!("{}U", scancode);
            };
        } else {
            if let Some(scancode) = key {
                print!("{}u", scancode);
            }; 
        }
        // if let Some(scancode) = key {
        //     print!("{}", scancode);
        // }; 
        
    
    
        //print!("{scancode}");
        // if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        //     if let Some(key) = keyboard.process_keyevent(key_event) {
        //         match key {
        //             DecodedKey::Unicode(character) => print!("{}", character),
        //             DecodedKey::RawKey(key) => print!("{:?}", key),
        //         }
        //     }
        // }
    }

}