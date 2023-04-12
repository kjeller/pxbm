use std::{borrow::BorrowMut, iter::Peekable, str::Lines};

use super::{util, MagicNumber, Netpbm, NetpbmFileType, NetpbmHeader};

pub struct Filetype;

impl NetpbmFileType for Filetype {
    fn parse(lines: &mut Peekable<Lines>, filetype: MagicNumber) -> Netpbm {
        let header = util::parse_netpbm_header(lines, filetype);

        let mut data: Vec<u8> = Vec::new();
        while let Some(line) = lines.next() {
            let mut line_vec = util::split_line_into_u8(line);
            data.append(line_vec.borrow_mut());
        }

        Netpbm {
            header,
            data,
        }
    }

    fn print(netpbm: &Netpbm, r: u8, g: u8, b: u8) {
        for i in 0..netpbm.header.height {
            for j in 0..netpbm.header.width {
                let pix = netpbm.data[(i * netpbm.header.width + j) as usize];

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
