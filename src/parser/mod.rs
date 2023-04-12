pub mod netpbm;
pub mod xbm;

pub trait Parser {
    fn print(&self, r: u8, g: u8, b: u8);
}
