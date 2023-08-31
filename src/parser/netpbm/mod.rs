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
