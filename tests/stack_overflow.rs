#![no_std]
#![no_main]

use core::panic::PanicInfo;
use Operating_System::gdt::init;




#[unsafe(no_mangle)]
pub extern "C" fn _start() -> !{
    init();
    stack_overflow();
    loop{}
}

#[allow(unconditional_recursion)]
fn stack_overflow(){ 
    stack_overflow();
    volatile::Volatile::new(0).read();
}



#[panic_handler]
fn panic(_info:&PanicInfo) -> !{
    loop{}
}
