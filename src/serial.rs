

use core::fmt::{self, Arguments};
use core::{format_args};
use uart_16550::{Config, Uart16550Tty, backend::PioBackend};
use lazy_static::lazy_static;
use spin::Mutex;
use core::default::Default;
lazy_static!{
    pub static ref SERIAL1: Mutex<Uart16550Tty<PioBackend>> = Mutex::new(unsafe{
        Uart16550Tty::new_port(0x3f8, Config::default()).expect("failed to initialize UART")
    });
}


#[doc(hidden)]
pub fn _print(args: Arguments){ 
    use core::fmt::Write;
    SERIAL1.lock().write_fmt(args).expect("Printing failed");
}

#[macro_export]
macro_rules! serial_println{
    ()=>{
        $crate::serial_print!("\n");
    };
    ($($arg:tt)*)=>{
        $crate::serial_print!("{}\n", format_args!($($arg)*));
    }
}

#[macro_export]
macro_rules! serial_print{
    ($($arg:tt)*)=>{
        $crate::serial::_print(format_args!($($arg)*));
    }
}
