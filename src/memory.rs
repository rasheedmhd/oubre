use x86_64::{
    structures::paging::{
        //page_table::FrameError,
        PageTable,
        OffsetPageTable,
    },
    registers::control::Cr3,
    VirtAddr,
    //PhysAddr,
};

pub unsafe fn init(physical_mem_offset: VirtAddr) 
-> OffsetPageTable<'static> 
{
    let level_4_table = active_level_4_table(physical_mem_offset);
    OffsetPageTable::new(level_4_table, physical_mem_offset)
}

pub unsafe fn active_level_4_table(physical_mem_offset: VirtAddr) 
-> &'static mut PageTable
{
    let (level_4_table_frame, _) = Cr3::read();
    let physical_addr = level_4_table_frame.start_address();
    let virt_addr = physical_mem_offset + physical_addr.as_u64();
    let page_table_ptr: *mut PageTable = virt_addr.as_mut_ptr();
    &mut *page_table_ptr // unsafe gymnastics 
}

// Translates a given virtual address into physical address or None if 
// the virtual address is not mapped
// pub unsafe fn translate_addr(addr: VirtAddr, physical_mem_offset: VirtAddr) 
// -> Option<PhysAddr> 
// {
//     safe_translate_addr(addr, physical_mem_offset)
// }

// Private function called by translate_addr
// fn safe_translate_addr(addr: VirtAddr, physical_mem_offset: VirtAddr)
// -> Option<PhysAddr>
// {
//     let (level_4_table_frame, _) = Cr3::read();
//     let table_indexes = [
//         addr.p4_index(),
//         addr.p3_index(),
//         addr.p2_index(),
//         addr.p1_index(),
//     ];
//     let mut frame = level_4_table_frame;

//     // traverse the multi-level page table
//     for &index in &table_indexes {
//         // convert the frame into a page table reference
//         let virt_addr = physical_mem_offset + frame.start_address().as_u64();
//         let table_ptr: *const PageTable = virt_addr.as_ptr();
//         let table = unsafe { &*table_ptr };

//         // read the page table entry and update the frame
//         let entry = &table[index];
//         frame = match entry.frame() {
//             Ok(frame) => frame,
//             Err(FrameError::FrameNotPresent) => return None,
//             Err(FrameError::HugeFrame) => panic!("huge pages not supported"),
//         };
//     }

//     // calculate the physical address by adding the page offset
//     Some(frame.start_address() + u64::from(addr.page_offset()))
// }