// GLOBAL DESCRIPTOR TABLE
// It comes from the days when segmented virtual memory was the only way to provide
// virtual address spaces. It has largely been out of favor with the introduction of paging
// Some systems like the x86 still use a combination of the Global Descriptor Table
// and paging to provide virtual address spaces

// the gdt is a table that contains segment descriptors
// the seg desc are in 3 types
//  1. NULL desc - must always be the first entry
//  2. call gate desc
//  3. task-state seg


use x86_64::{
    instructions::{
        tables::load_tss,
        segmentation::{
            CS, 
            Segment,
        },
    },
    VirtAddr,
    structures::{
        tss::TaskStateSegment,
        gdt::{
            SegmentSelector,
            GlobalDescriptorTable,
            Descriptor
        }
    },
};

use lazy_static::lazy_static;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 4;

lazy_static! {
    static ref GDT: ( GlobalDescriptorTable, Selectors ) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (gdt, Selectors { code_selector, tss_selector })
    };
}

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        // tss.interrupt_stack_table[0] = ...
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5; // 20,480
            // static mut STACK: [u8, 20,480]
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };
        tss
    };
}

pub fn init() {

    GDT.0.load(); // loading the null segment selector
    unsafe {
        CS::set_reg(GDT.1.code_selector);
        load_tss(GDT.1.tss_selector);
    }
    
}