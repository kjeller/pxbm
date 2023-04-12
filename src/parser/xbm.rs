use regex::Regex;

use super::Parser;

pub struct Xbm {
    width: u8,
    height: u8,
    data: Vec<u8>,
}

impl Xbm {
    pub fn parse(input: &str) -> Xbm {
        let width = Regex::new(r"_width (\d+)")
            .unwrap()
            .captures(input)
            .unwrap()[1]
            .parse::<u8>()
            .unwrap();
        let height = Regex::new(r"_height (\d+)")
            .unwrap()
            .captures(input)
            .unwrap()[1]
            .parse::<u8>()
            .unwrap();
        let data: Vec<u8> = Regex::new(r"0[xX]([0-9a-fA-F]+)")
            .unwrap()
            .captures_iter(input)
            .map(|f| u8::from_str_radix(&f[1], 16).unwrap())
            .collect();

        Xbm {
            data,
            width,
            height,
        }
    }
}

impl Parser for Xbm {
    /**
     * Prints xmb picture to console by reading bytewise (the bits) line by line
     * and changing background color (ANSI) for every bit that is 'highlighted'.
     */
    fn print(&self, r: u8, g: u8, b: u8) {
        // Width of bitmap in bytes
        let w_bytes: i32 = ((self.width + 7) / 8) as i32;

        for i in 0..self.height as i32 {
            for j in 0..w_bytes as i32 {
                // Get current byte with offset
                let byte = self.data[(i * w_bytes + j) as usize];

                for k in 0..8 as i32 {
                    // Loop through the bits
                    if byte & (1 << k) > 0 {
                        print!("\x1b[48;2;{r};{g};{b}m  ");
                    } else {
                        print!("\x1b[0m  ");
                    }
                }
            }
            println!();
        }
        print!("\x1b[0m  ");
    }
}
