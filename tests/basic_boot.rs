#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main="test_main"]

use core::panic::PanicInfo;

use Operating_System::println;


#[unsafe(no_mangle)]
pub extern "C" fn _start() -> !{
    test_main();

    loop{}
}


fn test_runner(tests: &[&dyn Operating_System::Testable]){
    Operating_System::test_runner(tests);
}
#[test_case]
fn test_println(){
    println!("Test");
}


#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    Operating_System::test_panic_handler(info)
}


