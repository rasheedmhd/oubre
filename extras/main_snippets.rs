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
                    
    //                 let phys_addr = entry.frame().unwrap().start_address();
    //                 let virt_addr = boot_info.physical_memory_offset + phys_addr.as_u64();
    //                 let ptr = VirtAddr::new(virt_addr).as_mut_ptr();
    //                 let l2_page_table: &PageTable = unsafe { &*ptr };
                    
    //                 for (i, entry) in l2_page_table.iter().enumerate() {
    //                     if !entry.is_unused() {
    //                         println!("L2 Entry: {}: {:?}", i, entry);
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // } 
  
  
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


    // If you are not failing a lot, you are probably not being as creative as you could be - you aren't stretching your imagination - John Backus, Creator of FORTRAN