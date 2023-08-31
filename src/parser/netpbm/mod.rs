mod pbm;
mod pgm;
mod ppm;
mod util;

use std::io::Write;
use anyhow::{Result, anyhow};

use parse_display::{Display, FromStr};

use crate::color::Color;

use super::Parser;

#[derive(Display, FromStr, PartialEq, Debug, Clone, Copy)]
pub enum MagicNumber {
    P1,
    P2,
    P3,
    P4,
    P5,
    P6,
}

#[derive(Debug)]
pub struct NetpbmHeader {
    width: u8,
    height: u8,
    bit_depth: u8,
    max_value: u32,
    filetype: MagicNumber,
}

#[derive(Debug)]
pub struct Netpbm {
    header: NetpbmHeader,
    data: Vec<u32>,
}

#[derive(Debug, thiserror::Error)]
pub enum NetpbmError {
    #[error("Invalid magic number. Should be one of P1, P2, P3, P4, P5, P6.")]
    InvalidMagicNumber,

    #[error("Could not parse {field}.")]
    ParseError {
        field: String,
    },
}

impl Netpbm {
    pub fn parse(input: &[u8]) -> Result<Netpbm> {
        let magic_number_bytes = &input.get(0..2).ok_or(anyhow!("Missing magic number."))?;
        let magic_number: MagicNumber =
            std::str::from_utf8(&magic_number_bytes)
            .map_err(|_| NetpbmError::InvalidMagicNumber)?
            .parse()
            .map_err(|_| NetpbmError::InvalidMagicNumber)?;
        
        util::parse(input, magic_number)
    }
}

impl<Writer: Write> Parser<Writer> for Netpbm {
    fn print(&self, color: Color, writer: &mut Writer) -> Result<()> {
        match self.header.filetype {
            MagicNumber::P1 => self.print_netpbm_p1(color, writer),
            MagicNumber::P4 => self.print_netpbm_p4(color, writer),
            MagicNumber::P2 | MagicNumber::P5 => self.print_netpgm(writer),
            MagicNumber::P3 | MagicNumber::P6 => self.print_netppm(writer),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_bitmap_p1() -> Result<()> {
        let input = br#"P1
        # A checkerboard picture
        8 2
        1 0 1 0 1 0 1 0
        0 1 0 1 0 1 0 1
        "#;

        let netpbm = Netpbm::parse(input)?;

        assert_eq!(netpbm.header.width, 8);
        assert_eq!(netpbm.header.height, 2);
        assert_eq!(netpbm.header.bit_depth, 1);
        assert_eq!(netpbm.header.filetype, MagicNumber::P1);
        assert_eq!(netpbm.data, vec![1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1]);

        let a = Color::new(Some((0xff, 0xee, 0xdd)));
        let b = Color::new(None);
        let mut output = Vec::<u8>::new();
        netpbm.print(a, &mut output)?;

        let expected = format!("{a}{b}{a}{b}{a}{b}{a}{b}\n{b}{a}{b}{a}{b}{a}{b}{a}\n");
        assert_eq!(output, expected.as_bytes());

        Ok(())
    }

    #[test]
    fn simple_bitmap_p2() -> Result<()> {
        let input = br#"P2
        # A simple picture
        3 2
        15
        0 7 15
        15 0 7
        "#;

        let netpbm = Netpbm::parse(input)?;

        assert_eq!(netpbm.header.width, 3);
        assert_eq!(netpbm.header.height, 2);
        assert_eq!(netpbm.header.max_value, 15);
        assert_eq!(netpbm.header.bit_depth, 8);
        assert_eq!(netpbm.header.filetype, MagicNumber::P2);
        assert_eq!(netpbm.data, vec![0, 7, 15, 15, 0, 7]);

        let a = Color::new(Some((0x00, 0x00, 0x00)));
        let b = Color::new(Some((0x77, 0x77, 0x77)));
        let c = Color::new(Some((0xff, 0xff, 0xff)));
        let mut output = Vec::<u8>::new();
        netpbm.print(a, &mut output)?;

        let expected = format!("{a}{b}{c}\n{c}{a}{b}\n");
        assert_eq!(output, expected.as_bytes());

        Ok(())
    }

    #[test]
    fn simple_bitmap_p3() -> Result<()> {
        let input = br#"P3
        # A simple picture
        3 2
        255
        255   0   0
        0 255   0
        0   0 255
        255 255   0 
        255 255 255
        0   0   0
        "#;

        let netpbm = Netpbm::parse(input)?;

        assert_eq!(netpbm.header.width, 3);
        assert_eq!(netpbm.header.height, 2);
        assert_eq!(netpbm.header.max_value, 255);
        assert_eq!(netpbm.header.bit_depth, 8);
        assert_eq!(netpbm.header.filetype, MagicNumber::P3);
        assert_eq!(netpbm.data, vec![255, 0, 0, 0, 255, 0, 0, 0, 255, 255, 255, 0, 255, 255, 255, 0, 0, 0]);

        let a = Color::new(Some((255,   0,   0)));
        let b = Color::new(Some((0, 255,   0)));
        let c = Color::new(Some((0,   0, 255)));
        let d = Color::new(Some((255, 255,   0 )));
        let e = Color::new(Some((255, 255, 255)));
        let f = Color::new(Some((0,   0,   0)));
        let mut output = Vec::<u8>::new();
        netpbm.print(a, &mut output)?;

        let expected = format!("{a}{b}{c}\n{d}{e}{f}\n");
        assert_eq!(output, expected.as_bytes());

        Ok(())
    }

    #[test]
    fn simple_bitmap_p4() -> Result<()> {
        let input = b"P4\n\
        # A checkerboard picture \n\
        8 2\n\
        \xaa\x55";

        let netpbm = Netpbm::parse(input)?;

        assert_eq!(netpbm.header.width, 8);
        assert_eq!(netpbm.header.height, 2);
        assert_eq!(netpbm.header.bit_depth, 1);
        assert_eq!(netpbm.header.filetype, MagicNumber::P4);
        assert_eq!(netpbm.data, vec![0xaa, 0x55]);

        let a = Color::new(Some((0xff, 0xee, 0xdd)));
        let b = Color::new(None);
        let mut output = Vec::<u8>::new();
        netpbm.print(a, &mut output)?;

        let expected = format!("{a}{b}{a}{b}{a}{b}{a}{b}\n{b}{a}{b}{a}{b}{a}{b}{a}\n");
        assert_eq!(output, expected.as_bytes());

        Ok(())
    }

    #[test]
    fn simple_bitmap_p5() -> Result<()> {
        let input = b"P5
        # A simple picture
        3 2
        15\n\
        \x00\x07\x0f\x0f\x00\x07";

        let netpbm = Netpbm::parse(input)?;

        assert_eq!(netpbm.header.width, 3);
        assert_eq!(netpbm.header.height, 2);
        assert_eq!(netpbm.header.max_value, 15);
        assert_eq!(netpbm.header.bit_depth, 8);
        assert_eq!(netpbm.header.filetype, MagicNumber::P5);
        assert_eq!(netpbm.data, vec![0, 7, 15, 15, 0, 7]);

        let a = Color::new(Some((0x00, 0x00, 0x00)));
        let b = Color::new(Some((0x77, 0x77, 0x77)));
        let c = Color::new(Some((0xff, 0xff, 0xff)));
        let mut output = Vec::<u8>::new();
        netpbm.print(a, &mut output)?;

        let expected = format!("{a}{b}{c}\n{c}{a}{b}\n");
        assert_eq!(output, expected.as_bytes());

        Ok(())
    }

    #[test]
    fn simple_bitmap_p6() -> Result<()> {
        let input = b"P6
        # A simple picture
        3 2
        255\n\
        \xff\x00\x00\x00\xff\x00\x00\x00\xff\xff\xff\x00\xff\xff\xff\x00\x00\x00";

        let netpbm = Netpbm::parse(input)?;

        assert_eq!(netpbm.header.width, 3);
        assert_eq!(netpbm.header.height, 2);
        assert_eq!(netpbm.header.max_value, 255);
        assert_eq!(netpbm.header.bit_depth, 8);
        assert_eq!(netpbm.header.filetype, MagicNumber::P6);
        assert_eq!(netpbm.data, vec![255, 0, 0, 0, 255, 0, 0, 0, 255, 255, 255, 0, 255, 255, 255, 0, 0, 0]);

        let a = Color::new(Some((255,   0,   0)));
        let b = Color::new(Some((0, 255,   0)));
        let c = Color::new(Some((0,   0, 255)));
        let d = Color::new(Some((255, 255,   0 )));
        let e = Color::new(Some((255, 255, 255)));
        let f = Color::new(Some((0,   0,   0)));
        let mut output = Vec::<u8>::new();
        netpbm.print(a, &mut output)?;

        let expected = format!("{a}{b}{c}\n{d}{e}{f}\n");
        assert_eq!(output, expected.as_bytes());

        Ok(())
    }
}