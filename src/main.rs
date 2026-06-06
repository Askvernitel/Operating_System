#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]


use core::{panic::PanicInfo};
use crate::vga_buffer::{Color, ColorCode, Writer };
use uart_16550::{Config, Uart16550Tty, backend::PioBackend};
mod vga_buffer;
mod serial;

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
    exit_qemu(QemuExitCode::SUCCESS);
}

#[repr(u32)]
pub enum QemuExitCode{
    SUCCESS = 0x10,
    FAILED = 0x11,
}


pub fn exit_qemu(qemu_exit_code:QemuExitCode){ 
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(qemu_exit_code as u32);
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> !{

    #[cfg(test)]
    test_main();

    println!("Hello World\n cool coolc oocl {}", 2);
    //panic!("Help");
    //exit_qemu(QemuExitCode::SUCCESS);
    loop{ 
    }
}
