
use x86_64::{
    structures::paging::PageTable,
    VirtAddr,
    PhysAddr
};
use core::{option::Option};


pub unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable{
    use x86_64::registers::control::Cr3;
    
    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    unsafe {&mut *page_table_ptr}
}



pub unsafe fn inner_translate_addr(addr:VirtAddr, physical_memory_offset:VirtAddr) -> Option<PhysAddr>{ 

    let indices = [addr.p1_index(), addr.p2_index(), addr.p3_index(), addr.p4_index()];
    let level_4_table = active_level_4_table(physical_memory_offset);


    for idx in indices{ 
    }
    Some(PhysAddr::new(123)) 
}
