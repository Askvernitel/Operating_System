#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(Operating_System::test_runner)]
#![reexport_test_harness_main = "test_main"]


use x86_64::{
    VirtAddr,
};

use core::{ panic::PanicInfo, arch::asm};
use Operating_System::memory::*;
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
//    println!("Hello World\n  {}", 2);
    Operating_System::init();
    

    //use x86_64::registers::control::Cr3;
    //let (level_4_table, _) = Cr3::read();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = unsafe { active_level_4_table(phys_mem_offset)};


    for (i, entry) in l4_table.iter().enumerate(){
        if !entry.is_unused(){
            println!("L4 entry: {}, {:?}", i, entry);
        }
    }

    //println!("level 4 page table: {:?}", level_4_table.start_address());
//    let invalid_mem = 0x206ada as *mut u8;
  //  unsafe{
 //       println!("{}", *invalid_mem);
        //invalid_mem.write(42);
   // }
//    x86_64::instructions::interrupts::int3();
    #[cfg(test)]
    test_main();   
    hlt_loop(); 
}
