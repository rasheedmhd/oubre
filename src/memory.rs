#![no_std]
// #![no_main]

use x86_64::{
    structures::paging::PageTable,
    registers::control::Cr3,
    VirtAddr,
};

pub unsafe fn active_level_4_table(physical_mem_offset: VirtAddr) 
-> &'static mut PageTable
{
    let (level_4_table_frame, _) = Cr3::read();
    let physical_addr = level_4_table_frame.start_address();
    let virt_addr = physical_mem_offset + physical_addr.as_u64();
    let page_table_ptr: *mut PageTable = virt_addr.as_mut_ptr();
    &mut *page_table_ptr // unsafe gymnastics 
}