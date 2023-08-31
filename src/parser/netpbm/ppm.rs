use std::io::Write;

use anyhow::Result;

use crate::{color::Color, pxbm_write, pxbm_writeln};

use super::Netpbm;

impl Netpbm {
    pub fn print_netppm(&self, writer: &mut dyn Write) -> Result<()> {
        for i in 0..self.header.height {
            for j in (0..self.header.width * 3).step_by(3) {
                let (r, g, b): (u32, u32, u32);

                if self.header.bit_depth == 16 {
                    // clamping to 0-255, but still supporting 0-65535
                    r = self.data[(i * 3 * self.header.width + j) as usize] * 65535
                        / self.header.max_value
                        / 255;
                    g = self.data[(i * 3 * self.header.width + j + 1) as usize] * 65535
                        / self.header.max_value
                        / 255;
                    b = self.data[(i * 3 * self.header.width + j + 2) as usize] * 65535
                        / self.header.max_value
                        / 255;
                } else {
                    r = self.data[(i * 3 * self.header.width + j) as usize] * 255 / self.header.max_value;
                    g = self.data[(i * 3 * self.header.width + j + 1) as usize] * 255 / self.header.max_value;
                    b = self.data[(i * 3 * self.header.width + j + 2) as usize] * 255 / self.header.max_value;
                }

                pxbm_write!(writer, "{}", Color::new(Some((r as u8, g as u8, b as u8))))?;
            }
            pxbm_writeln!(writer)?;
        }
        Ok(())
    }
}