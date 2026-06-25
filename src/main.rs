#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(Operating_System::test_runner)]
#![reexport_test_harness_main = "test_main"]



use core::{ panic::PanicInfo, arch::asm};
use crate::vga_buffer::{Color, ColorCode, Writer, WRITER, BUFFER_HEIGHT };
use uart_16550::{Config, Uart16550Tty, backend::PioBackend};
mod vga_buffer;
mod serial;



#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    println!("{}", _info);

    loop{}
}

pub fn get_rsp() -> u64{
    let rsp:u64;
    unsafe{ 
        asm!("mov {}, rsp", out(reg) rsp)
    }
    rsp
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> !{
    println!("Hello World\n  {}", 2);
    
    Operating_System::init();

//    x86_64::instructions::interrupts::int3();
    #[cfg(test)]
    test_main();   
    loop{ 
    }
}
