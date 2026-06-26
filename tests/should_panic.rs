#![no_std]
#![no_main]

use core::panic::PanicInfo;

use Operating_System::{QemuExitCode, exit_qemu, serial_println, serial_print };


#[unsafe(no_mangle)]
pub extern "C" fn _start() -> !{
    should_fail();
    loop{}
}

fn should_fail(){ 
    serial_print!("should_fail\t");
    assert_eq!(0, 1);
}

#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::SUCCESS);
    loop{}
}




