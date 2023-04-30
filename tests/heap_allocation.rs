#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(oubre_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{
    entry_point,
    BootInfo,
};
use core::panic::PanicInfo;

use oubre_os::{
    allocator::{self, HEAP_SIZE},
    memory::{
        self,
        BootInfoFrameAllocator,
    },
};
use x86_64::VirtAddr;

entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {
    
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    test_main();
    loop {}
}


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    oubre_os::test_panic_handler(info);
}

use alloc::boxed::Box;

#[test_case]
fn test_allocation() {
    let test_heap_num = Box::new(23);
    let test_heap_num2 = Box::new(24);
    assert_eq!(*test_heap_num, 23);
    assert_eq!(*test_heap_num2, 24);
}

#[test_case]
fn large_vec() {
    use alloc::vec::Vec;
    let mut vec = Vec::new();
    let n = 1000;
    for i in 1..n {
        vec.push(i)
    }
    assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2);
}


#[test_case]
fn many_boxes() {
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
}