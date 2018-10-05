use core::fmt;

use spin::Mutex;
use volatile::Volatile;

use self::color::ColorCode;
use self::buffer::Buffer;

pub use self::color::Color;
pub use self::writer::Writer;

mod color;
mod writer;
mod buffer;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
const VGA_FRAMEBUFFER_ADDRESS: usize = 0xb8000;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::new(
        Color::Yellow,
        Color::Black,
        unsafe { Buffer::from_address(VGA_FRAMEBUFFER_ADDRESS) }
    ));
}

macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::print(format_args!($($arg)*)));
}

macro_rules! println {
    () => (print!("\n"));
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

pub fn print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}
