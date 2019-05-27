#![allow(dead_code)]

//! Color managemement.
//!
//! # Example
//!
//! ```rust
//! use termion::color;
//!
//! fn main() {
//!     println!("{}Red", color::Fg(color::Red));
//!     println!("{}Blue", color::Fg(color::Blue));
//!     println!("{}Back again", color::Fg(color::Reset));
//! }
//! ```

use numtoa::NumToA;
use std::fmt;
use std::fmt::Debug;

/// A terminal color.
pub trait Color: Debug {
    /// Write the foreground version of this color.
    fn write_fg(&self, f: &mut fmt::Formatter) -> fmt::Result;
    /// Write the background version of this color.
    fn write_bg(&self, f: &mut fmt::Formatter) -> fmt::Result;
}

macro_rules! derive_color {
    ($doc:expr, $name:ident, $value:expr) => {
        #[doc = $doc]
        #[derive(Copy, Clone, Debug)]
        pub struct $name;

        impl Color for $name {
            #[inline]
            fn write_fg(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str(self.fg_str())
            }

            #[inline]
            fn write_bg(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str(self.bg_str())
            }
        }

        impl $name {
            #[inline]
            /// Returns the ANSI escape sequence as a string.
            pub fn fg_str(&self) -> &'static str { csi!("38;5;", $value, "m") }

            #[inline]
            /// Returns the ANSI escape sequences as a string.
            pub fn bg_str(&self) -> &'static str { csi!("48;5;", $value, "m") }
        }
    };
}

derive_color!("Black.", Black, "0");
derive_color!("Red.", Red, "1");
derive_color!("Green.", Green, "2");
derive_color!("Yellow.", Yellow, "3");
derive_color!("Blue.", Blue, "4");
derive_color!("Magenta.", Magenta, "5");
derive_color!("Cyan.", Cyan, "6");
derive_color!("White.", White, "7");
derive_color!("High-intensity light black.", LightBlack, "8");
derive_color!("High-intensity light red.", LightRed, "9");
derive_color!("High-intensity light green.", LightGreen, "10");
derive_color!("High-intensity light yellow.", LightYellow, "11");
derive_color!("High-intensity light blue.", LightBlue, "12");
derive_color!("High-intensity light magenta.", LightMagenta, "13");
derive_color!("High-intensity light cyan.", LightCyan, "14");
derive_color!("High-intensity light white.", LightWhite, "15");

impl<'a> Color for &'a Color {
    #[inline]
    fn write_fg(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (*self).write_fg(f)
    }

    #[inline]
    fn write_bg(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (*self).write_bg(f)
    }
}

/// An arbitrary ANSI color value.
#[derive(Clone, Copy, Debug)]
pub struct AnsiValue(pub u8);

impl AnsiValue {
    /// 216-color (r, g, b ≤ 5) RGB.
    pub fn rgb(r: u8, g: u8, b: u8) -> AnsiValue {
        debug_assert!(
            r <= 5,
            "Red color fragment (r = {}) is out of bound. Make sure r ≤ 5.",
            r
        );
        debug_assert!(
            g <= 5,
            "Green color fragment (g = {}) is out of bound. Make sure g ≤ 5.",
            g
        );
        debug_assert!(
            b <= 5,
            "Blue color fragment (b = {}) is out of bound. Make sure b ≤ 5.",
            b
        );

        AnsiValue(16 + 36 * r + 6 * g + b)
    }

    /// Grayscale color.
    ///
    /// There are 24 shades of gray.
    pub fn grayscale(shade: u8) -> AnsiValue {
        // Unfortunately, there are a little less than fifty shades.
        debug_assert!(
            shade < 24,
            "Grayscale out of bound (shade = {}). There are only 24 shades of \
             gray.",
            shade
        );

        AnsiValue(0xE8 + shade)
    }
}

impl AnsiValue {
    /// Returns the ANSI sequence as a string.
    pub fn fg_string(self) -> String {
        let mut x = [0u8; 20];
        let x = self.0.numtoa_str(10, &mut x);
        [csi!("38;5;"), x, "m"].concat()
    }

    /// Returns the ANSI sequence as a string.
    pub fn bg_string(self) -> String {
        let mut x = [0u8; 20];
        let x = self.0.numtoa_str(10, &mut x);
        [csi!("48;5;"), x, "m"].concat()
    }
}

impl Color for AnsiValue {
    #[inline]
    fn write_fg(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.fg_string())
    }

    #[inline]
    fn write_bg(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.bg_string())
    }
}

/// A truecolor RGB.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgb(pub u8, pub u8, pub u8);

impl Rgb {
    /// Returns the ANSI sequence as a string.
    pub fn fg_string(self) -> String {
        let (mut x, mut y, mut z) = ([0u8; 20], [0u8; 20], [0u8; 20]);
        let (x, y, z) = (
            self.0.numtoa_str(10, &mut x),
            self.1.numtoa_str(10, &mut y),
            self.2.numtoa_str(10, &mut z),
        );

        [csi!("38;2;"), x, ";", y, ";", z, "m"].concat()
    }

    /// Returns the ANSI sequence as a string.
    pub fn bg_string(self) -> String {
        let (mut x, mut y, mut z) = ([0u8; 20], [0u8; 20], [0u8; 20]);
        let (x, y, z) = (
            self.0.numtoa_str(10, &mut x),
            self.1.numtoa_str(10, &mut y),
            self.2.numtoa_str(10, &mut z),
        );

        [csi!("48;2;"), x, ";", y, ";", z, "m"].concat()
    }
}

impl Color for Rgb {
    #[inline]
    fn write_fg(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.fg_string())
    }

    #[inline]
    fn write_bg(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.bg_string())
    }
}

/// Reset colors to defaults.
#[derive(Debug, Clone, Copy)]
pub struct Reset;

const RESET_FG: &str = csi!("39m");
const RESET_BG: &str = csi!("49m");

impl Reset {
    /// Returns the ANSI sequence as a string.
    pub fn fg_str(self) -> &'static str {
        RESET_FG
    }
    /// Returns the ANSI sequence as a string.
    pub fn bg_str(self) -> &'static str {
        RESET_BG
    }
}

impl Color for Reset {
    #[inline]
    fn write_fg(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(RESET_FG)
    }

    #[inline]
    fn write_bg(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(RESET_BG)
    }
}

/// A foreground color.
#[derive(Debug, Clone, Copy)]
pub struct Fg<C: Color>(pub C);

impl<C: Color> fmt::Display for Fg<C> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.write_fg(f)
    }
}

/// A background color.
#[derive(Debug, Clone, Copy)]
pub struct Bg<C: Color>(pub C);

impl<C: Color> fmt::Display for Bg<C> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.write_bg(f)
    }
}
