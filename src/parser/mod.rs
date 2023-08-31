use crate::color::Color;

pub mod netpbm;
pub mod xbm;
pub mod xpm;

pub trait Parser {
    fn print(&self, color: Color);
}
