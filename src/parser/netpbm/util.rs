use std::{iter::Peekable, str::Lines};

use super::{MagicNumber, NetpbmHeader};

pub fn parse_netpbm_header(lines: &mut Peekable<Lines>, filetype: MagicNumber) -> NetpbmHeader {
    // TODO check what type here P1 doesn't use max value or bit dpeth
    // TODO use u16 for max value
    let (width, height, max_value, bit_depth): (u8, u8, u8, u8);

    if let Some(line) = lines.next() {
        let line_vec = split_line_into_u8(line);
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
                Some(mv) => match mv.parse::<u8>() {
                    Ok(mv) => {
                        if mv > 255 {
                            (mv, 16)
                        } else {
                            (mv, 8)
                        }
                    }
                    Err(_) => panic!("Parse error - expected max value"),
                },
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

pub fn split_line_into_u8(line: &str) -> Vec<u8> {
    line.split_whitespace()
        .into_iter()
        .map(|f| match f.parse::<u8>() {
            Ok(w) => w,
            Err(_) => panic!("Parse error - expected unsigned integer on line"),
        })
        .collect()
}
