use std::io::Write;

use anyhow::Result;

use crate::{color::Color, pxbm_write, pxbm_writeln};

use super::Netpbm;

impl Netpbm {
    pub fn print_netpgm(&self, writer: &mut dyn Write) -> Result<()> {
        for i in 0..self.header.height {
            for j in 0..self.header.width {
                let pix: u32;

                if self.header.bit_depth == 16 {
                    pix = self.data[(i * self.header.width + j) as usize] * 65535 / self.header.max_value / 255;
                } else {
                    pix = (self.data[(i * self.header.width + j) as usize] * 255 / self.header.max_value) as u32;
                }

                pxbm_write!(writer, "{}", Color::new(Some((pix as u8, pix as u8, pix as u8))))?;
            }
            pxbm_writeln!(writer)?;
        }
        Ok(())
    }
}