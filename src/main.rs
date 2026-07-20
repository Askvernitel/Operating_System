#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(Operating_System::test_runner)]
#![reexport_test_harness_main = "test_main"]


use x86_64::{
    VirtAddr,
};

use core::{ panic::PanicInfo, arch::asm};
use Operating_System::{memory::{self, *}, translate_addr};
use crate::vga_buffer::{Color, ColorCode, Writer, WRITER, BUFFER_HEIGHT };
use Operating_System::hlt_loop;
use bootloader::{BootInfo, entry_point};
use uart_16550::{Config, Uart16550Tty, backend::PioBackend};
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
    println!("Hello World\n  {}", 2);
    Operating_System::init();
    
    
   
    let addresses = [
        0x8000,
        0x0000,
        0x123123123,
        boot_info.physical_memory_offset,
    ];


    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);


    let mut mapper = unsafe { memory::init(phys_mem_offset)};
    let mut frame_allocator = memory::EmptyFrameAllocator;



    for &addr in &addresses{
        let virt = VirtAddr::new(addr);

        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }

    #[cfg(test)]
    test_main();   
    hlt_loop(); 
}
