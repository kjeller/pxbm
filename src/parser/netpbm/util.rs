use std::iter::Peekable;

use itertools::Itertools;
use anyhow::{Result, Context};

use crate::PxbmError;

use super::{MagicNumber, Netpbm, NetpbmHeader};

fn normalize_ascii(s: &str) -> String {
    s
        .lines()
        .map(|line| line.split('#').next().unwrap()) // Remove from '#' to end of line
        .join("\n")
        .split_ascii_whitespace()
        .join(" ") // Replace all whitespaces with single space
}

fn skip_comments_and_whitespace<'a>(iter: &mut Peekable<impl Iterator<Item = &'a u8>>) {
    while let Some(byte) = iter.peek() {
        if byte.is_ascii_whitespace() {
            iter.next();
        } else if byte == &&b'#' {
            while iter.peek() != Some(&&b'\n') {
                iter.next();
            }
        } else {
            break;
        }
    }
}

fn next_field<'a>(iter: &mut Peekable<impl Iterator<Item = &'a u8>>) -> Result<Vec<u8>> {
    let mut v = Vec::new();

    skip_comments_and_whitespace(iter);
    while let Some(byte) = iter.peek() {
        if !byte.is_ascii_whitespace() {
            let byte = iter.next().unwrap();
            v.push(*byte);
        } else {
            break;
        }
    }
    if !v.is_empty() {
        Ok(v)
    } else {
        Err(PxbmError::Eof.into())
    }
}

fn normalize_binary(bytes: &[u8], filetype: MagicNumber) -> Result<Vec<u8>> {
    let mut iter = bytes.iter().peekable();

    let magic_number = next_field(&mut iter)?;
    let width = next_field(&mut iter)?;
    let height = next_field(&mut iter)?;
    let max_value = match filetype {
        MagicNumber::P1 | MagicNumber::P2 | MagicNumber::P3 => None,
        MagicNumber::P4 | MagicNumber::P5 | MagicNumber::P6 => {
            Some(next_field(&mut iter)?)
        },
    };

    let mut v = Vec::new();
    v.extend_from_slice(&magic_number);
    v.push(b' ');
    v.extend(width);
    v.push(b' ');
    v.extend(height);
    v.push(b' ');
    if let Some(max_value) = max_value {
        v.extend(max_value);
        v.push(b' ');
    }
    v.extend(iter);
    Ok(v)
}

fn parse_ascii(input: &[u8], magic_number: MagicNumber) -> Result<Netpbm> {
    let input = normalize_ascii(std::str::from_utf8(input).map_err(|_| PxbmError::BadUtf8)?);
    let mut words = input.split_ascii_whitespace();
    let _magic_number = words
        .next()
        .ok_or(PxbmError::Eof)?;
    let width = words
        .next()
        .ok_or(PxbmError::Eof)?
        .parse::<u8>()
        .context("Could not parse width.")?;
    let height = words
        .next()
        .ok_or(PxbmError::Eof)?
        .parse::<u8>()
        .context("Could not parse height.")?;
    let max_value: Option<u32> = if magic_number != MagicNumber::P1 {
        Some(
            words
                .next()
                .ok_or(PxbmError::Eof)?
                .parse::<u32>()
                .context("Could not parse max value.")?
        )
    } else {
        None
    };
    let bit_depth = if let Some(max) = max_value {
        if max > 255 {
            16u8
        } else {
            8u8
        }
    } else {
        1u8
    };
    let header = NetpbmHeader {
        width,
        height,
        bit_depth,
        max_value: max_value.unwrap_or(1),
        filetype: magic_number,
    };

    let data = words.map(|x| x.parse().context(format!("Expected numbers in raster data - found \"{x}\""))).collect::<Result<Vec<_>>>()?;
    Ok(Netpbm { header, data })
}

fn parse_binary(input: &[u8], magic_number: MagicNumber) -> Result<Netpbm> {
    let input = normalize_binary(input, magic_number).context("Could not normalize file.")?;
    let num_header_elements = if magic_number == MagicNumber::P4 { 3 } else { 4 };
    let mut iter = input.splitn(num_header_elements + 1, |byte| byte == &b' ');
    let _magic_number = iter.next().unwrap();
    let width = std::str::from_utf8(iter.next().unwrap())?.parse::<u8>().context("Could not parse width.")?;
    let height = std::str::from_utf8(iter.next().unwrap())?.parse::<u8>().context("Could not parse height.")?;
    let max_value = if magic_number != MagicNumber::P4 {
        Some(std::str::from_utf8(iter.next().unwrap())?.parse::<u32>().context("Could not parse max value.")?)
    } else {
        None
    };
    let bit_depth = if let Some(max) = max_value {
        if max > 255 {
            16u8
        } else {
            8u8
        }
    } else {
        1u8
    };
    let header = NetpbmHeader {
        width,
        height,
        bit_depth,
        max_value: max_value.unwrap_or(1),
        filetype: magic_number,
    };
    let data = if bit_depth > 8 {
        iter.next().unwrap().chunks(2).map(|chunk| {
            u16::from_be_bytes([chunk[0], *chunk.get(1).unwrap_or(&0u8)]) as u32
        })
        .collect()
    } else {
        iter.next().unwrap().iter().map(|x| *x as u32).collect()
    };
    Ok(Netpbm { header, data })
}

pub fn parse(input: &[u8], magic_number: MagicNumber) -> Result<Netpbm> {
    match magic_number {
        MagicNumber::P1 |
        MagicNumber::P2 |
        MagicNumber::P3 => parse_ascii(input, magic_number),
        MagicNumber::P4 |
        MagicNumber::P5 |
        MagicNumber::P6 => parse_binary(input, magic_number),
    }
}
