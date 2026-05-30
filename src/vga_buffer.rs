use core::fmt::{self, Arguments};

use volatile::Volatile;
use fmt::Write;
use lazy_static::lazy_static;
use spin::Mutex;
pub enum Color{ 
    BLACK = 0,
    BLUE = 1,
    GREEN = 2,
    CYAN = 3,
    RED = 4,
    MAGENTA = 5,
    BROWN = 6,
    LIGHT_GRAY = 7,
    DARK_GRAY = 8,
    LIGHT_BLUE = 9,
    LIGHT_GREEN = 10,
    LIGHT_CYAN = 11,
    LIGHT_RED = 12,
    PINK = 13,
    YELLOW = 14,
    WHITE = 15,
}

lazy_static!{
    pub static ref WRITER:Mutex<Writer> = Mutex::new(Writer::new());
}

#[macro_export]
macro_rules! println{
    () => {print!("\n")};
    ($($arg:tt)*) =>{
        $crate::print!("{}\n", format_args!($($arg)*))
    };
}
#[macro_export]
macro_rules! print{
    ($($arg:tt)*) =>{
        $crate::vga_buffer::_print(format_args!($($arg)*))
    };
}

pub fn _print(fmt_args: Arguments){ 
    use fmt::Write;
    WRITER.lock().write_fmt(fmt_args).unwrap();
}

#[derive(Copy,Clone)]
pub struct ColorCode(u8);

impl ColorCode{ 
    pub fn new(bg_color:Color, fg_color:Color) -> Self{ 
        ColorCode(
            ((bg_color as u8) << 4) | fg_color as u8
        )
    }
}


#[repr(C)]
#[derive(Copy,Clone)]
pub struct ScreenChar{ 
    ascii_char: u8,
    color_code: ColorCode,
}

impl ScreenChar{ 
    pub fn new(char:u8, color_code:ColorCode)->Self{ 
        ScreenChar{ 
            ascii_char:char,
            color_code:color_code,
        }
    }
}


const BUFFER_WIDTH:usize = 80;
const BUFFER_HEIGHT:usize = 25;

pub struct Buffer{ 
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}


pub struct Writer{ 
    column_position: usize,
    color_code: ColorCode,
    screen_buffer: &'static mut Buffer,
}


impl Write for Writer{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

impl Writer{
    pub fn new()->Self{
        Writer{
            column_position:0,
            screen_buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
            color_code:ColorCode::new(Color::BLACK, Color::WHITE),
        }
    }
     

    pub fn write_byte(&mut self, char:u8){ 
        match char{
            b'\n' =>{
                self.new_line()
            },
            _ => {
                if(BUFFER_WIDTH <= self.column_position){
                    self.new_line();
                }

                self.screen_buffer.chars[BUFFER_HEIGHT-1][self.column_position].write(
                    ScreenChar::new(char, self.color_code)
                );
                self.column_position+=1;
            }

        }
    }

    pub fn new_line(&mut self){

        for row in 1..BUFFER_HEIGHT{
            for col in 0..BUFFER_WIDTH{
                let char = self.screen_buffer.chars[row][col].read();
                self.screen_buffer.chars[row-1][col].write(char);
            }
        }
        self.clear_row();
        self.column_position = 0;
    }

    pub fn clear_row(&mut self){
        for col in 0..BUFFER_WIDTH{
            self.screen_buffer.chars[BUFFER_HEIGHT-1][col].write(
                ScreenChar{
                    ascii_char:b' ',
                    color_code:self.color_code,
                }
            );
        }
    }
    pub fn write_string(&mut self, s:&str){
        for b in s.bytes(){

            match b{ 
                0x20..=0x7e | b'\n' => {
                    self.write_byte(b);
                }
                _ => {
                    self.write_byte(0xfe);
                }
            }

        }
    }

    pub fn set_color_code(&mut self, color_code:ColorCode){
        self.color_code = color_code;
    }
}
