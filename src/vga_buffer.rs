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

#[derive(Clone)]
pub struct ColorCode(u8);

impl ColorCode{ 
    pub fn new(bg_color:Color, fg_color:Color) -> Self{ 
        ColorCode(
            ((bg_color as u8) << 4) | fg_color as u8
        )
    }
}


#[repr(C)]
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
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}


pub struct Writer{ 
    column_position: usize,
    row_position: usize,
    color_code: ColorCode,
    screen_buffer: &'static mut Buffer,
}

impl Writer{
    pub fn new()->Self{
        Writer{
            column_position:0,
            row_position:0,
            screen_buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
            color_code:ColorCode::new(Color::BLACK, Color::WHITE),
        }
    }
     

    pub fn write(&mut self, char:ScreenChar){ 
        match char.ascii_char{
            b'\n' =>{
                self.new_line()
            },
            _ => {
                if(BUFFER_WIDTH <= self.column_position){
                    self.new_line();
                }
                self.screen_buffer.chars[self.row_position][self.column_position] = char;
                self.column_position+=1;
            }

        }
        //self.vga_buffer.offset(self.offset); //= char.char;
    }

    pub fn new_line(&mut self){
        self.row_position += 1;
        self.column_position = 0;
    }
    pub fn write_str(&mut self, s:&str){
        for b in s.bytes(){
            self.write(ScreenChar::new(b, self.color_code.clone()));
        }
    }

    pub fn set_color_code(&mut self, color_code:ColorCode){
        self.color_code = color_code;
    }
}
