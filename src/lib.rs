use image::Luma;

/// Converts from image::Luma (grayscale) to ascii. For Rgb values, first convert them to luma
/// In the future, further functions could be added to support colorised ascii art
///
/// # Examples
///
/// Basic Usage

/// ```
/// Convert Luma<u8> (i.e. intensity 0-255) to a singel ascii character
pub fn luma_to_ascii(luma: &Luma<u8>, invert: bool) -> char {
    let ascii_string = r##"$$@@B%8&WM##*ooahkbdpqwmZO0QLCJUYXzcvunxrjft/\|()1{}[]??--_++~~<>i!lI;::,,""^^``''.. "##;

    let scale_factor: f64 = luma[0] as f64 / 255.0;
    let position = ascii_string.len() as f64 * scale_factor;
    let index = if invert {
        std::cmp::max(position.round() as usize, ascii_string.len() - 1)
    } else {
        let x = (position.round() + 1.0) as usize;
        if x > ascii_string.len() {
            0
        } else {
            ascii_string.len() - x
        }
    };
    
    ascii_string.chars().skip(index).next().unwrap()
}

/// Stores the characters and dimensions of an image that has been converted to ascii art
/// This implements display for us so that it can easily be printed
pub struct AsciiArt {
    asciis: Vec<char>,
    dimensions: (usize, usize),
}

impl AsciiArt {
    /// Create a new AsciiArt struct from a Vec<char> and dimensions
    pub fn new(asciis: Vec<char>, dimensions: (u32, u32)) -> Self {
        AsciiArt { asciis, dimensions: (dimensions.0 as usize, dimensions.1 as usize) }
    }
}

impl std::fmt::Display for AsciiArt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for y in 0..self.dimensions.1 {
            for x in 0..self.dimensions.0 {
                // print twice to stretch the image horizontally, counteracting the aspect ratio of characters in a terminal
                write!(f, "{}", self.asciis[x + y * self.dimensions.0])?;
                write!(f, "{}", self.asciis[x + y * self.dimensions.0])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
