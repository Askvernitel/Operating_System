use core::{alloc::GlobalAlloc,  ptr::null_mut};

use linked_list_allocator::LockedHeap;

pub struct Dummy;

#[global_allocator]
pub static ALLOCATOR:LockedHeap = LockedHeap::empty();

unsafe impl GlobalAlloc for Dummy{
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        null_mut()
    } 
    
    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        panic!("No dealloc")        
    }

}