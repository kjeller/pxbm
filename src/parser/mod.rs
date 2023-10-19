use std::io::Write;

use crate::color::Color;
use anyhow::Result;

pub mod netpbm;
pub mod xbm;
pub mod xpm;

pub trait Parser<Writer: Write> {
    fn print(&self, color: Color, writer: &mut Writer) -> Result<()>;
}
