#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]


use core::panic::PanicInfo;
use crate::vga_buffer::{Color, ColorCode, Writer };
use core::fmt::Write;
mod vga_buffer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    println!("{}", _info);
    loop{}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]){
    println!("Running {} Tests Yippie", tests.len());

    for test in tests{ 
        test();
    }
}


#[unsafe(no_mangle)]
pub extern "C" fn _start() -> !{

    #[cfg(test)]
    test_main();

    println!("Hello World\n cool coolc oocl {}", 2);
    panic!("Help");
    loop{ 
    }
}
