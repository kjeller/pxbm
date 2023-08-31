mod pbm;
mod pgm;
mod ppm;
mod util;

use std::{iter::Peekable, str::Lines};
use anyhow::{Result, anyhow};

use parse_display::{Display, FromStr};

use crate::color::Color;

use super::Parser;

pub trait NetpbmFileType {
    fn parse(input: &mut Peekable<Lines>, filetype: MagicNumber) -> Netpbm;
    fn print(netpbm: &Netpbm, r: u8, g: u8, b: u8);
}

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

impl Parser for Netpbm {
    fn print(&self, color: Color) {
        match self.header.filetype {
            MagicNumber::P1 => pbm::print_netpbm_p1(self, color),
            MagicNumber::P2 => pgm::print_netpgm(self),
            MagicNumber::P3 => ppm::print_netppm(self),
            MagicNumber::P4 => pbm::print_netpbm_p4(self, color),
            MagicNumber::P5 => pgm::print_netpgm(self),
            MagicNumber::P6 => ppm::print_netppm(self),
        }
    }
}
