mod p1;
mod p2;
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
    max_value: u8,
    filetype: MagicNumber,
}

pub struct Netpbm {
    header: NetpbmHeader,
    data: Vec<u8>,
}

impl Netpbm {
    pub fn parse(input: &str) -> Netpbm {
        let filetype: MagicNumber;
        let mut lines = input.lines().peekable();

        util::skip_comments_and_whitespace(lines.borrow_mut());
        if let Some(ft) = lines.next() {
            if let Ok(ft) = ft.parse::<MagicNumber>() {
                filetype = ft;
            } else {
                panic!("Parse error - could not parse filetype");
            }
        } else {
            panic!("Parse error - expected line with filetype");
        }
        util::skip_comments_and_whitespace(lines.borrow_mut());

        match filetype {
            MagicNumber::P1 => p1::Filetype::parse(lines.borrow_mut(), filetype),
            MagicNumber::P2 => p2::Filetype::parse(lines.borrow_mut(), filetype),
            MagicNumber::P3 => !todo!(),
            MagicNumber::P4 => !todo!(),
            MagicNumber::P5 => !todo!(),
            MagicNumber::P6 => !todo!(),
            _ => panic!("Unexpected filetype"),
        }
    }
}

impl Parser for Netpbm {
    fn print(&self, r: u8, g: u8, b: u8) {
        match self.header.filetype {
            MagicNumber::P1 => p1::Filetype::print(self, r, g, b),
            MagicNumber::P2 => p2::Filetype::print(self, r, g, b),
            MagicNumber::P3 => !todo!(),
            MagicNumber::P4 => !todo!(),
            MagicNumber::P5 => !todo!(),
            MagicNumber::P6 => !todo!(),
            _ => panic!("Unexpected filetype"),
        }
    }
}
