#![no_std]
#![no_main]

use core::panic::PanicInfo;

use crate::vga_buffer::{Color, ColorCode, Writer};
mod vga_buffer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop{}
}




static HELLO: &[u8] = b"Hello World!";
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> !{

    let mut writer = Writer::new();
    writer.set_color_code(ColorCode::new(Color::BLACK, Color::BLUE));

        
    loop{ 
    }
}
