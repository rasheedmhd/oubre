use volatile::Volatile;
use core::fmt;

#[allow(dead_code)] // disabling compiler warnings for unused codes
// like for structs, the compiler implements some Traits for enums, 
// but we have to ask first using the #[derive()] attribute
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// in memory rust stores C-style enums as integers the smallest integer value
// that can accomodate the variant is used but we can tell rust the 
// the integer value that we want it to use with #[repr()] attribute
// here we tell rust to store our enum variants in memory as u8 integers
#[repr(u8)]
pub enum Color {
    Black = 0x0,
    Blue = 0x1,
    Green = 0x2,
    Cyan = 0x3,
    Red = 0x4,
    Magenta = 0x5,
    Brown = 0x6,
    LightGray = 0x7,
    DarkGray = 0x8,
    LightBlue = 0x9,
    LightGreen = 0xa,
    LightCyan = 0xb,
    LightRed = 0xc,
    Pink = 0xd,
    Yellow = 0xe,
    White = 0xf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// repr(transparent) is used on structs or enums with only 1 variant
// with a non-zero sized type.
// it guarantees that the whole struct/enum's layout and ABI is that of
// the single field/variant.
//https://doc.rust-lang.org/nomicon/other-reprs.html#reprtransparent
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> Self {
        ColorCode((background as u8) << 4 | (foreground as u8)) 
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// structs layout in Rust is undefined, we use the #[repr(C)] attribute so that struct is laid out
// exactly like in C which guarantees the right field ordering
#[repr(C)]
// the text buffer is a 2D array
// ScreenChar represents a single item to print in the array
// which is the buffer
struct ScreenChar {
    char_to_print: u8,
    color_code: ColorCode,
}

impl ScreenChar {
    fn new(char: u8) -> Self {
        ScreenChar {
            char_to_print: char,
            color_code: ColorCode::new(Color::Green, Color::Black),
        }
    }
}
// Defining the bounderies of the text buffer - 2d array 
const VGA_BUFFER_HEIGHT: usize = 25;
const VGA_BUFFER_WIDTH: usize = 80;

// Guarantees that Buffer is laid out in memory exactly like its one Field, chars
#[repr(transparent)] 
struct Buffer {
    //chars: [[ScreenChar; VGA_BUFFER_WIDTH]; VGA_BUFFER_HEIGHT],
    // wrapping ScreenChar with Volatile which uses read/write_volatile under the
    // hood to prevent the compiler from optimizing the write to the buffer
    // away since we are writing only once without reading.
    chars: [[Volatile<ScreenChar>; VGA_BUFFER_WIDTH]; VGA_BUFFER_HEIGHT],
}

pub struct Screen {
    cursor_position: usize,
    //color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Screen {
    pub fn print_byte(&mut self, byte: u8) {
        match byte {
            // if the byte value is a '\n' we call new_line() 
            b'\n' => self.new_line(),
            // if byte has a value we check if the current array line is full 
            // with characters
            // if it is full we create a new line.
            byte => {
                if self.cursor_position >= VGA_BUFFER_WIDTH {
                    self.new_line();
                }

                let row = VGA_BUFFER_HEIGHT - 1;
                let col = self.cursor_position;

                //let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar::new(byte));
                // writing a new ScreenChar to the buffer
                // self.buffer.chars[row][col].write(ScreenChar {
                //     char_to_print: byte,
                //     color_code,
                // });
                self.cursor_position += 1;
            }
        }
    }
    // to write strings we first convert it into bytes and write 
    // byte by byte 
    pub fn print_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                // 0x20 = space (in hex)
                // 0x7e = ~ (in hex)
                // we want to print anything starting from space to ~ inclusively
                // or a new line character \n
                0x20..=0x7e | b'\n' => self.print_byte(byte),
                // not part of printable ASCII range
                // We pass everything that is not ASCII printable to the write_byte
                // method defined above to be printed out as a block(■)- Oxfe in hex
                _ => self.print_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) { 
        for row in 1..VGA_BUFFER_HEIGHT {
            for col in 0..VGA_BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row -1][col].write(character);
            }
        }
        self.clear_row(VGA_BUFFER_HEIGHT - 1);
        self.cursor_position = 0;
     }

     fn clear_row(&mut self, row: usize) { /* TODO */ }
}

impl fmt::Write for Screen {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.print_string(s);
        Ok(())
    }
}

pub fn print_something() {
    use core::fmt::Write;
    let mut screen_printer = Screen {
        cursor_position: 0,
        //color_code: ColorCode::new(Color::LightGreen, Color::Black),
        // set the buffer to the VGA text buffer address as a mutable raw pointer
        // dereference it - meaning return the memory address pointed to by the raw pointer,
        // and borrow it mutably - so you can read/write to to.
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    screen_printer.print_byte(b'H');
    screen_printer.print_string("ello! ");
    //writer.write_string(" Wörld!");
    write!(screen_printer, "The numbers are {} and {}", 42, 1.0/3.0).unwrap();
}