#![no_std]
#![no_main]

use core::panic::PanicInfo;
use crate::vga_buffer::{Color, ColorCode, Writer };
use core::fmt::Write;
mod vga_buffer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    println!("{}", _info);
    loop{}
}



#[unsafe(no_mangle)]
pub extern "C" fn _start() -> !{

    println!("Hello World\n cool coolc oocl {}", 2);
    panic!("Help");
    loop{ 
    }
}
