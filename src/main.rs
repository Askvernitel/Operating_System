#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]


use core::{panic::PanicInfo};
use crate::vga_buffer::{Color, ColorCode, Writer, WRITER, BUFFER_HEIGHT };
use uart_16550::{Config, Uart16550Tty, backend::PioBackend};
mod vga_buffer;
mod serial;


#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    println!("{}", _info);
    //exit_qemu(QemuExitCode::FAILED);

    loop{}
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    serial_println!("Fancy formatting [error]\n");
    serial_println!("Error: {}\n", _info);
    exit_qemu(QemuExitCode::FAILED);
    loop{}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]){
    serial_println!("Running {} Tests", tests.len());
    for test in tests{ 
        test.run();
    }
    exit_qemu(QemuExitCode::SUCCESS);
}

#[test_case]
pub fn test_println(){ 
    println!("Testing VGA Printing");
}


#[test_case]
pub fn test_println_bulk(){
    for i in 1..300{
        println!("Testing VGA Printing {}", i);
    }
}


#[test_case]
pub fn test_println_output(){
    let write_str = "Hello World How Are You?";
    println!("{}", write_str);
    for (i, val) in write_str.chars().enumerate(){
        let out = WRITER.lock().screen_buffer.chars[BUFFER_HEIGHT-2][i].read().ascii_char as char;
        assert_eq!(val, out);
    }
}

trait Testable{ 
    fn run(&self) -> ();
}

impl<T:Fn()> Testable for T{ 
    fn run(&self) -> () {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[OK]");
    }
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
