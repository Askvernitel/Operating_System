#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![feature(abi_x86_interrupt)]
#![reexport_test_harness_main = "test_main"]

use core::{assert_eq, iter::Iterator, ops::Fn, panic::PanicInfo, prelude::v1::test_case, format_args};
use core::fmt::Write;
use crate::vga_buffer::{Color, ColorCode, Writer, WRITER, BUFFER_HEIGHT };
use spin::Mutex;
use volatile::Volatile;
pub mod serial;
pub mod vga_buffer;
pub mod interrupts;
pub mod gdt;

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

pub fn hlt_loop() -> !{
    loop{
        x86_64::instructions::hlt();
    }
}


pub fn test_panic_handler(info: &PanicInfo) -> !{
    serial_println!("Fancy formatting [error]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::FAILED);
    hlt_loop();
}

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
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(||{
        let mut writer = WRITER.lock();
        writeln!(writer, "\n{}",write_str).expect("writeln failed");
        for (i, val) in write_str.chars().enumerate(){
            let out = writer.screen_buffer.chars[BUFFER_HEIGHT-2][i].read().ascii_char as char;
            assert_eq!(val, out);
        }
    });
}




pub trait Testable{ 
    fn run(&self) -> ();
}

impl<T:Fn()> Testable for T{ 
    fn run(&self) -> () {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[OK]");
    }
}

#[cfg(test)]
#[panic_handler]
fn panic(info:&PanicInfo) -> !{
    test_panic_handler(info)
}

pub fn init(){
    gdt::init();
    interrupts::init_idt();
    unsafe{ interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

#[cfg(test)]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> !{
    init();
    test_main();
    hlt_loop();
}
