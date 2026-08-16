
use x86_64::{
    structures::paging::{mapper::MapToError, FrameAllocator, Mapper, OffsetPageTable, Page, PageTable, PageTableFlags, PhysFrame, Size4KiB}, PhysAddr, VirtAddr
};

use bootloader::{bootinfo::MemoryMap, BootInfo};
use bootloader::bootinfo::MemoryRegionType;
use core::{option::Option, option::Option::Some, option::Option::None};

use crate::{allocator::ALLOCATOR, println};



pub const HEAP_START:usize = 0x4444_4444_0000;
pub const HEAP_SIZE:usize = 100*1024;

pub struct EmptyFrameAllocator;

unsafe impl FrameAllocator<Size4KiB> for EmptyFrameAllocator{
    fn allocate_frame(&mut self) -> Option<PhysFrame>{
        None
    }
}


pub struct BootInfoFrameAllocator{
    memory_map:&'static MemoryMap,
    next: usize,
}
impl BootInfoFrameAllocator{ 
    pub unsafe fn init(memory_map:&'static MemoryMap)->Self{
        BootInfoFrameAllocator{
            memory_map,
            next:0,
        }
    }

    pub fn usable_frames(&self) -> impl Iterator<Item = PhysFrame>{ 
        let regions = self.memory_map.iter();
        //println!("{:?}", regions);
        regions.filter(|r| r.region_type == MemoryRegionType::Usable)
        .map(|r| r.range.start_addr()..r.range.end_addr())
        .flat_map(|r| r.step_by(4096))
        .map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}
unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator{
    fn allocate_frame(&mut self) -> Option<PhysFrame>{
        let phys_frame = self.usable_frames().nth(self.next);
        self.next+=1; 
        phys_frame
    }   
}

pub unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable{
    use x86_64::registers::control::Cr3;
    
    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    unsafe {&mut *page_table_ptr}
}


pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static>{
    unsafe{ 
        let level_4_table = active_level_4_table(physical_memory_offset);
        OffsetPageTable::new(level_4_table, physical_memory_offset)
    }
}

pub unsafe fn inner_translate_addr(addr:VirtAddr, physical_memory_offset:VirtAddr) -> Option<PhysAddr>{ 
    use x86_64::structures::paging::page_table::FrameError;
    use x86_64::registers::control::Cr3;
    
    let indices = [addr.p4_index(), addr.p3_index(), addr.p2_index(), addr.p1_index()];
    let (level_4_table_frame, _) = Cr3::read();

    let mut frame = level_4_table_frame;
    for &idx in &indices{ 
        let virt = physical_memory_offset + frame.start_address().as_u64();
        let table_ptr:*const PageTable= virt.as_ptr();
        let table = unsafe{&*table_ptr};
        
        let entry = &table[idx];

        frame = match entry.frame(){
            Ok(frame) => frame,
            Err(FrameError::FrameNotPresent) => return None,
            Err(FrameError::HugeFrame) => panic!("huge pages not supported"),
        }
    }

    Some(frame.start_address() + u64::from(addr.page_offset()))
}
pub fn init_heap(mapper: &mut impl Mapper<Size4KiB>, frame_allocator: &mut impl FrameAllocator<Size4KiB>) -> Result<(),MapToError<Size4KiB>>{
    
    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end =  heap_start + (HEAP_SIZE as u64 - 1u64) ;
        let heap_start_page :Page<Size4KiB>= Page::containing_address(heap_start);
        let heap_end_page :Page<Size4KiB>=  Page::containing_address(heap_end);

        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    for page in page_range{    
        let frame = frame_allocator
        .allocate_frame()
        .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe{
            mapper.map_to(page, frame, flags, frame_allocator)?.flush();
        };
    }
    unsafe{
        ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }
    Ok(())
    //.ok_or()?;
}

fn create_example_mapping(
    page: Page,
    mapper: &mut OffsetPageTable,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
    ){ 
    use x86_64::structures::paging::PageTableFlags as Flags;

    let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
    let flags = Flags::PRESENT | Flags::WRITABLE;

    let map_to_result = unsafe { 
        //FIXME: Not Safe
        mapper.map_to(page, frame, flags, frame_allocator)
    };
    map_to_result.expect("map_to failed").flush();
}
