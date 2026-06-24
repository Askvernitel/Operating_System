#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(Operating_System::test_runner)]
#![reexport_test_harness_main = "test_main"]



use core::{ panic::PanicInfo};
use crate::vga_buffer::{Color, ColorCode, Writer, WRITER, BUFFER_HEIGHT };
use uart_16550::{Config, Uart16550Tty, backend::PioBackend};
mod vga_buffer;
mod serial;



#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    println!("{}", _info);

    loop{}
}



#[unsafe(no_mangle)]
pub extern "C" fn _start() -> !{
//    println!("Hello World\n cool coolc oocl {}", 2);
    
    Operating_System::init();


    unsafe{
        let addr = (0xdeadbeef as *mut u8);

        println!("{}", *addr);
    }
    x86_64::instructions::interrupts::int3();
    #[cfg(test)]
    test_main();   
    loop{ 
    }
}
