#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(oubre_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(non_snake_case)]

extern crate alloc;

use core::panic::PanicInfo;
use oubre_os::{
    memory,
    memory::{
        // active_level_4_table,
        // translate_addr,
        // EmptyFrameAllocator,
        BootInfoFrameAllocator,
    },
    gdt, 
        interrupts, 
        //print, 
        println, 
        allocator,
    };

use bootloader::{
        BootInfo,
        entry_point
    };

use x86_64::{
    instructions::interrupts as hardware_interrupts,
    //registers::control::Cr3,
    //PhysAddr,
    VirtAddr,
    structures::paging::{
        //PageTable,
        Page,
        //Translate,
    }, 
};

use alloc::{
    boxed::Box,
    vec,
    vec::Vec,
    rc::Rc
};


entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {

    println!("
    Hi, I am Oubre OS
    
    Works on x86_64 arch machines
    version 0.0.1
    Display size: 80 * 25
    version: v0.00.01
    
    ----------------------------------
    Creator: Rasheed Starlet Maverick
    Copy Left @ www.rasheedstarlet.com
    ");

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    let mut mapper = unsafe {
        memory::init(phys_mem_offset)
    };

    // let mut frame_allocator = EmptyFrameAllocator;
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    // map an unused page
    let page = Page::containing_address(VirtAddr::new(0x89980000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string 'New!' to the screen through the new mapping 
    // let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    // unsafe {
    //     page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)
    // };


    // let addresses = [
    //     // the identity-mapped vga buffer page
    //     0xb8000,
    //     // some code page
    //     0x201008,
    //     // some stack page
    //     0x0100_0020_1a10,
    //     // virtual address mapped to physical address 0
    //     boot_info.physical_memory_offset,
    // ];
    
    // for &address in &addresses {
    //     let virt_addr = VirtAddr::new(address);
    //     let phys_addr = mapper.translate_addr(virt_addr);
    //     println!("{:?} -> {:?}", virt_addr, phys_addr);
    // }

    allocator::init_heap(&mut mapper, &mut frame_allocator)
    .expect("heap initialization failed");

    // allocating a number on the heap
    let heap_num = Box::new(41);
    println!("heap number at {:p}", heap_num);
    
    // creating a dynamically sized vector
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());
    
    // creating a reference countr vector that will be freed with when reaches 0
    let ref_counted = Rc::new(vec![1,2,3]);
    let cloned_ref = ref_counted.clone();
    println!("current reference count is {}", Rc::strong_count(&cloned_ref));
    let cloned_ref2 = cloned_ref.clone();
    println!("current reference count is {}", Rc::strong_count(&cloned_ref2));
    core::mem::drop(ref_counted);
    println!("current reference count is {} now", Rc::strong_count(&cloned_ref));


    
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
