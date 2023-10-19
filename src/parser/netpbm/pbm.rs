use std::io::Write;

use crate::{color::Color, pxbm_write, pxbm_writeln};
use anyhow::Result;

use super::Netpbm;

impl Netpbm {
    pub fn print_netpbm_p1(&self, color: Color, writer: &mut dyn Write) -> Result<()> {
        for i in 0..self.header.height {
            for j in 0..self.header.width {
                let pix = self.data[(i * self.header.width + j) as usize];

                if pix > 0 {
                    pxbm_write!(writer, "{color}")?;
                } else {
                    pxbm_write!(writer, "{}", Color::transparent())?;
                }
            }
            pxbm_writeln!(writer)?;
        }
        Ok(())
    }

    pub fn print_netpbm_p4(&self, color: Color, writer: &mut dyn Write) -> Result<()> {
        let width_in_bytes = (self.header.width + 7) / 8;
        for i in 0..self.header.height {
            for j in 0..width_in_bytes {
                let byte = self.data[(i * width_in_bytes + j) as usize];

                for k in (0..=7).rev() {
                    if byte & (1 << k) > 0 {
                        pxbm_write!(writer, "{color}")?;
                    } else {
                        pxbm_write!(writer, "{}", Color::transparent())?;
                    }
                }
            }
            pxbm_writeln!(writer)?;
        }
        Ok(())
    }
}