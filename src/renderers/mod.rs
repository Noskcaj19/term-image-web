use crate::termion::color::{self, Bg, Fg, Rgb};
use std::fmt::Write;
pub mod block;
pub mod braille;

/// Convert full color rgb to 256 color
fn rgb_to_ansi(color: color::Rgb) -> color::AnsiValue {
    let r = (u16::from(color.0) * 5 / 255) as u8;
    let g = (u16::from(color.1) * 5 / 255) as u8;
    let b = (u16::from(color.2) * 5 / 255) as u8;
    color::AnsiValue(16 + 36 * r + 6 * g + b)
}

trait DrawableCell {
    fn print(&self, truecolor: bool, stdout: &mut impl Write) {
        if truecolor {
            self.print_truecolor(stdout);
        } else {
            self.print_ansi(stdout);
        }
    }

    fn print_truecolor(&self, stdout: &mut impl Write);
    fn print_ansi(&self, stdout: &mut impl Write);
}
