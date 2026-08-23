use core::{alloc::GlobalAlloc, ptr::null_mut};

use alloc::boxed::Box;

use super::Locked;




pub struct LinkedListAllocator{ 
    head:ListNode,
}


impl LinkedListAllocator{
    pub const fn new() -> Self{
        LinkedListAllocator{
            head:ListNode::empty(),
        }
    }
}

unsafe impl GlobalAlloc for Locked<LinkedListAllocator> {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        null_mut()
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
    }
}
pub struct ListNode{
    alloc_start:usize,
    size:usize,
    next: Option<Box<ListNode>>,
}


impl ListNode {
    pub const fn empty()->Self{
        ListNode{
            alloc_start:0,
            size: 0,
            next: None,
        }

    }
    fn new(alloc_start:usize, size:usize) -> Self{
        ListNode{
            alloc_start:alloc_start,
            size: size,
            next: None,
        }
    }

    fn add(&mut self, list_node:Box<ListNode>){
        self.next = Some(list_node);
    }
    
}