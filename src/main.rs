#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(oubre_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(non_snake_case)]

use core::panic::PanicInfo;
use oubre_os::{
        memory,
        gdt, 
        interrupts, 
        //print, 
        println,
    };

use bootloader::{
        BootInfo,
        entry_point
    };

use x86_64::{
    instructions::interrupts as hardware_interrupts,
    //registers::control::Cr3,
    VirtAddr,
    structures::paging::{
        //PageTable,
        Translate,
    },
};


entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {

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

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    let mapper = unsafe {
        memory::init(phys_mem_offset)
    };

    let addresses = [
        // the identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x201008,
        // some stack page
        0x0100_0020_1a10,
        // virtual address mapped to physical address 0
        boot_info.physical_memory_offset,
    ];
    for &address in &addresses {
        let virt_addr = VirtAddr::new(address);
        // let phys_addr = unsafe { 
        //     translate_addr(virt_addr, phys_mem_offset) 
        // };
        let phys_addr = mapper.translate_addr(virt_addr);
        println!("{:?} -> {:?}", virt_addr, phys_addr);
    }

    // let l4_table = unsafe { 
    //     active_level_4_table(phys_mem_offset) 
    // };

    // for (i, entry) in l4_table.iter().enumerate() {
    //     if !entry.is_unused() {
    //         println!("L4 entry {}: {:?}", i, entry);

    //         // retrieve physical address from entry and convert it
    //         let phys_addr = entry.frame().unwrap().start_address();
    //         let virt_addr = phys_addr.as_u64() + boot_info.physical_memory_offset;
    //         let ptr = VirtAddr::new(virt_addr).as_mut_ptr();
    //         let l3_table: &PageTable = unsafe { &*ptr };

    //         for (i, entry) in l3_table.iter().enumerate() {
    //             if !entry.is_unused() {
    //                 println!("L3 Entry: {}: {:?}", i, entry);
    //             }
    //         }
    //     }
    // }

    init_descriptor_tables();
    init_PICs();
    
    fn init_descriptor_tables() {
        gdt::init();
        interrupts::init_idt();
    }

    // unsafe {   

    //     read 
    //     let x = *(0x2049c4 as *mut &'static str);
    //     println!("read worked");

    //     write -> Triggers Page Fault
    //     *(0x2049c4 as *mut &'static str) = "Break Me, Mdf";
    //     println!("write worked");
    // }

    // let (level_4_page_table, _) = Cr3::read();
    // println!("Level 4 Page Table: {:?}", level_4_page_table.start_address());
    
    fn init_PICs() {
        unsafe {
            interrupts::PICS.lock().initialize();
            // executes the sti(set interrupt) instruction to enable external interrupts
            hardware_interrupts::enable();

            
        }
    }
    
    #[cfg(test)]
    test_main();

    oubre_os::hlt_loop();
    
}


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
