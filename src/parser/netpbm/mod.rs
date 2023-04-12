mod pbm;
mod pgm;
mod ppm;
mod util;

use std::{borrow::BorrowMut, iter::Peekable, str::Lines};

use parse_display::{Display, FromStr};

use super::Parser;

pub trait NetpbmFileType {
    fn parse(input: &mut Peekable<Lines>, filetype: MagicNumber) -> Netpbm;
    fn print(netpbm: &Netpbm, r: u8, g: u8, b: u8);
}

#[derive(Display, FromStr, PartialEq, Debug)]
pub enum MagicNumber {
    P1,
    P2,
    P3,
    P4,
    P5,
    P6,
}

pub struct NetpbmHeader {
    width: u8,
    height: u8,
    bit_depth: u8,
    max_value: u32,
    filetype: MagicNumber,
}

pub struct Netpbm {
    header: NetpbmHeader,
    data: Vec<u32>,
}

impl Netpbm {
    pub fn parse(input: &str) -> Netpbm {
        let filetype: MagicNumber;
        let mut lines = input.lines().peekable();

        util::skip_comments_and_whitespace(lines.borrow_mut());
        if let Some(ft) = lines.next() {
            if let Ok(ft) = ft[0..2].parse::<MagicNumber>() {
                filetype = ft;
            } else {
                panic!("Parse error - could not parse filetype");
            }
        } else {
            panic!("Parse error - expected line with filetype");
        }
        util::skip_comments_and_whitespace(lines.borrow_mut());
        util::parse(lines.borrow_mut(), filetype)
    }
}

impl Parser for Netpbm {
    fn print(&self, r: u8, g: u8, b: u8) {
        match self.header.filetype {
            MagicNumber::P1 => pbm::print_netpbm_p1(self, r, g, b),
            MagicNumber::P2 => pgm::print_netpgm(self),
            MagicNumber::P3 => ppm::print_netppm(self),
            MagicNumber::P4 => pbm::print_netpbm_p4(self, r, g, b),
            MagicNumber::P5 => pgm::print_netpgm(self),
            MagicNumber::P6 => ppm::print_netppm(self),
        }
    }
}
