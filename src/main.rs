#![no_std] 
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(Operating_System::test_runner)]
#![reexport_test_harness_main = "test_main"]


extern crate alloc;

use x86_64::{
    VirtAddr,
};

use core::{ panic::PanicInfo, arch::asm};
use Operating_System::{memory::{self, *}, translate_addr};
use crate::vga_buffer::{Color, ColorCode, Writer, WRITER, BUFFER_HEIGHT };
use Operating_System::hlt_loop;
use bootloader::{BootInfo, entry_point};
use uart_16550::{Config, Uart16550Tty, backend::PioBackend};
use alloc::boxed::Box;

mod vga_buffer;
mod serial;

#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    println!("{}", _info);
    
    hlt_loop();
}

pub fn get_rsp() -> u64{
    let rsp:u64;
    unsafe{ 
        asm!("mov {}, rsp", out(reg) rsp)
    }
    rsp
}

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> !{
    use x86_64::{structures::paging::Translate};
    use x86_64::{structures::paging::Page, VirtAddr};
    Operating_System::init();
    
    //let x = Box::new(10);
    let addresses = [
        0x8000,
        0x0000,
        0x123123123,
        boot_info.physical_memory_offset,
    ];

    let phys_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_memory_offset)};
    let mut frame_allocator = unsafe{ memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };

    init_heap(&mut mapper, &mut frame_allocator);

    let x = Box::new(10);
    //let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));



//    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);
    
/* 
    let page_ptr:*mut u64 = page.start_address().as_mut_ptr();
    
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};
*/
    /*



    for &addr in &addresses{
        let virt = VirtAddr::new(addr);

        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }*/
    println!("Hello World\n  {}", 2);
    #[cfg(test)]
    test_main();   
    hlt_loop(); 
}
