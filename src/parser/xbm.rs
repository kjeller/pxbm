use std::io::Write;

use regex::Regex;
use anyhow::Result;

use crate::{color::Color, pxbm_write, pxbm_writeln};

use super::Parser;

pub struct Xbm {
    width: u8,
    height: u8,
    data: Vec<u8>,
}

impl Xbm {
    pub fn parse(input: &[u8]) -> Xbm {
        let input = std::str::from_utf8(input).unwrap();
        let width = Regex::new(r"_width (\d+)")
            .unwrap()
            .captures(input)
            .unwrap()[1]
            .parse::<u8>()
            .unwrap();
        let height = Regex::new(r"_height (\d+)")
            .unwrap()
            .captures(input)
            .unwrap()[1]
            .parse::<u8>()
            .unwrap();
        let data: Vec<u8> = Regex::new(r"0[xX]([0-9a-fA-F]+)")
            .unwrap()
            .captures_iter(input)
            .map(|f| u8::from_str_radix(&f[1], 16).unwrap())
            .collect();

        Xbm {
            data,
            width,
            height,
        }
    }
}

impl<Writer: Write> Parser<Writer> for Xbm {
    // Prints xbm picture to console by reading bytewise (the bits) line by line
    // and changing background color (ANSI) for every bit that is 'highlighted'.
    fn print(&self, color: Color, writer: &mut Writer) -> Result<()> {
        // Width of bitmap in bytes
        let w_bytes: i32 = ((self.width + 7) / 8) as i32;

        for i in 0..self.height as i32 {
            for j in 0..w_bytes as i32 {
                // Get current byte with offset
                let byte = self.data[(i * w_bytes + j) as usize];

                for k in 0..8 as i32 {
                    // Loop through the bits
                    if byte & (1 << k) > 0 {
                        pxbm_write!(writer, "{color}")?;
                    } else {
                        pxbm_write!(writer, "{}", Color::new(None))?;
                    }
                }
            }
            pxbm_writeln!(writer)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_bitmap() -> Result<()> {
        // Checkerboard pattern
        // 10101010
        // 01010101
        let input = b"
            #define test_width 8
            #define test_height 2
            static unsigned char test_bits[] = {
                0x55, 0xAA
            };
        ";
        let xbm = Xbm::parse(input);
        assert_eq!(xbm.width, 8);
        assert_eq!(xbm.height, 2);
        assert_eq!(xbm.data, vec![0x55, 0xAA]);

        let w = Color::new(Some((255, 255, 255)));
        let b = Color::new(None);
        let mut output = Vec::<u8>::new();
        xbm.print(w, &mut output)?;
        
        let expected = format!("{w}{b}{w}{b}{w}{b}{w}{b}\n{b}{w}{b}{w}{b}{w}{b}{w}\n");
        assert_eq!(output, expected.as_bytes());

        Ok(())
    }
}