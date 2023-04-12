use std::{borrow::BorrowMut, iter::Peekable, str::Lines};

use super::{util, MagicNumber, Netpbm, NetpbmFileType, NetpbmHeader};

pub struct Filetype;

impl NetpbmFileType for Filetype {
    fn parse(lines: &mut Peekable<Lines>, filetype: MagicNumber) -> Netpbm {
        let (width, height): (u8, u8);

        if let Some(line) = lines.next() {
            let line_vec = util::split_line_into_u8(line);
            width = line_vec[0];
            height = line_vec[1];
        } else {
            panic!("Parse error - expected width and height");
        }

        util::skip_comments_and_whitespace(lines);

        let mut data: Vec<u8> = Vec::new();
        while let Some(line) = lines.next() {
            let mut line_vec = util::split_line_into_u8(line);
            data.append(line_vec.borrow_mut());
        }

        let header = NetpbmHeader {
            bit_depth: 1,
            max_value: 1,
            extension: ".pbm".to_string(),
            filetype,
        };

        Netpbm {
            header,
            width,
            height,
            data,
        }
    }

    fn print(netpbm: &Netpbm, r: u8, g: u8, b: u8) {
        for i in 0..netpbm.height {
            for j in 0..netpbm.width {
                let pix = netpbm.data[(i * netpbm.width + j) as usize];

                if pix > 0 {
                    print!("\x1b[48;2;{r};{g};{b}m  ");
                } else {
                    print!("\x1b[0m  ");
                }
            }
            println!();
        }

        print!("\x1b[0m  ");
    }
}
