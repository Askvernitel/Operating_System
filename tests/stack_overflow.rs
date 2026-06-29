#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use lazy_static::lazy_static;
use core::panic::PanicInfo;
use Operating_System::{QemuExitCode, exit_qemu, gdt::{DOUBLE_FAULT_IST_INDEX, init}, hlt_loop, serial_print, serial_println};

lazy_static!{
    pub static ref TEST_IDT:InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe{
            idt.double_fault.set_handler_fn(test_double_fault_handler)
                .set_stack_index(DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

extern "x86-interrupt" fn test_double_fault_handler(_stack_frame:InterruptStackFrame, _error_code:u64) -> !{
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::SUCCESS);
    hlt_loop();
}


#[unsafe(no_mangle)]
pub extern "C" fn _start() -> !{
    serial_print!("stack_overflow test...\t");
    init();
    init_test_idt();
    stack_overflow();

    panic!("stack_overflow test continued");
}

#[allow(unconditional_recursion)]
fn stack_overflow(){ 
    stack_overflow();
}

fn init_test_idt(){
    TEST_IDT.load();
}

#[panic_handler]
fn panic(_info:&PanicInfo) -> !{
    hlt_loop();
}
