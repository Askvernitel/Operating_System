#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main="test_main"]

extern crate alloc;
use core::panic::PanicInfo;

use alloc::{boxed::Box, vec::Vec};
use bootloader::{entry_point, BootInfo};
use Operating_System::{allocator::{self, HEAP_SIZE}, hlt_loop, memory, println };

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> !{
    use x86_64::{structures::paging::Translate};
    use x86_64::{structures::paging::Page, VirtAddr};
    Operating_System::init();
    
    let phys_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_memory_offset)};
    let mut frame_allocator = unsafe{ memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator);
    test_main();
    loop{}
}


fn test_runner(tests: &[&dyn Operating_System::Testable]){
    Operating_System::test_runner(tests);
}

#[test_case]
fn simple_allocation(){
    let al1 = Box::new(50);
    let al2 = Box::new(51);

    assert_eq!(*al1, 50);
    assert_eq!(*al2, 51);
}

#[test_case]
fn large_allocation(){
    let sz = 1000;
    let mut v :Vec<u64>= Vec::new();
    for i in 1..sz{ 
        v.push(i);
    }
    assert_eq!(v.iter().sum::<u64>(), (sz * (sz-1))/2)
}


#[test_case]
fn many_boxes(){
    for i in 1..HEAP_SIZE{
        let bx = Box::new(i);
        assert_eq!(*bx, i);
    }
}
#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    Operating_System::test_panic_handler(info)
}


