use std::{borrow::BorrowMut, iter::Peekable, str::Lines};

use super::{MagicNumber, Netpbm, NetpbmHeader};

fn parse_netpbm_header(lines: &mut Peekable<Lines>, filetype: MagicNumber) -> NetpbmHeader {
    let (width, height, bit_depth): (u8, u8, u8);
    let max_value: u32;

    if let Some(line) = lines.next() {
        let line_vec = split_line_into_vec(line);
        width = line_vec[0];
        height = line_vec[1];
    } else {
        panic!("Parse error - expected width and height");
    }

    (max_value, bit_depth) = match filetype {
        MagicNumber::P1 | MagicNumber::P4 => (1, 1),
        _ => {
            skip_comments_and_whitespace(lines);

            match lines.next() {
                Some(mv) => {
                    println!("parsed {mv}");
                    let first_integer: Vec<&str> = mv.split_whitespace().collect();
                    if let Some(first_integer) = first_integer.first() {
                        match first_integer.parse::<u32>() {
                            Ok(mv) => {
                                if mv > 255 {
                                    (mv, 16)
                                } else {
                                    (mv, 8)
                                }
                            }
                            Err(_) => panic!("Parse error - expected max value"),
                        }
                    } else {
                        panic!("Parse error - expected max value");
                    }
                }
                _ => panic!("Parse error - expected max value line"),
            }
        }
    };

    skip_comments_and_whitespace(lines);

    NetpbmHeader {
        width,
        height,
        bit_depth,
        max_value,
        filetype,
    }
}

pub fn parse(lines: &mut Peekable<Lines>, filetype: MagicNumber) -> Netpbm {
    let header = parse_netpbm_header(lines, filetype);

    let mut data: Vec<u32> = Vec::new();
    while let Some(line) = lines.next() {
        let mut line_vec = split_line_into_vec(line);
        data.append(line_vec.borrow_mut());
    }

    Netpbm { header, data }
}

pub fn skip_comments_and_whitespace(lines: &mut Peekable<Lines>) {
    while let Some(line) = lines.peek() {
        if line.char_indices().count() > 1 {
            match &line[0..1] {
                "#" | "\0" | "\n" => {
                    lines.next();
                    continue;
                }
                _ => break,
            }
        } else {
            lines.next();
            continue;
        }
    }
}

fn split_line_into_vec<T: std::str::FromStr>(line: &str) -> Vec<T> {
    line.split_whitespace()
        .into_iter()
        .filter(|&f| f.chars().all(char::is_numeric))
        .map(|f| match f.parse::<T>() {
            Ok(w) => w,
            Err(_) => panic!("Parse error - expected unsigned integer on line"),
        })
        .collect()
}
