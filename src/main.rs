use std::io::{Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use image::{self, Luma};
use anyhow::{anyhow, Result};

use ascii::{AsciiArt, luma_to_ascii};

fn main() -> Result<()> {
    let filepath = std::env::args().skip(1).next().ok_or(anyhow!("No filename provided"))?;

    let image = image::open(&filepath)?.resize(100, 50, image::imageops::FilterType::CatmullRom).to_luma8();
    let dimensions = image.dimensions();

    let pixels = image.enumerate_pixels().map(|(_x, _y, &pixel)| pixel).collect::<Vec<Luma<u8>>>();
    let asciis = pixels.iter().map(|l| luma_to_ascii(l, false)).collect::<Vec<char>>();
    let ascii_art = AsciiArt::new(asciis, (dimensions.0 as u32, dimensions.1 as u32));
    
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
    writeln!(&mut stdout, "{}", &ascii_art)?;

    Ok(())
}
