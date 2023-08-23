pub mod netpbm;
pub mod xbm;
pub mod xpm;

pub trait Parser {
    fn print(&self, r: u8, g: u8, b: u8);
}
