use self::xbm::Xbm;

pub mod xbm;

pub enum Format {
//    Netbm,
    Xbm,
}

pub trait PrintableFormat {
    fn print(&self);
}

// Detects and creates a format type from CLI argument input
pub struct FormatParser {
    pub format: Format,
}

impl FormatParser {
    pub fn parse(&self, input: &str) -> impl PrintableFormat {
        // TODO Decide whether to use netbm() or xbm()
        match self.format {
  //          Format::Netbm => Xbm::parse(input),
            Format::Xbm => Xbm::parse(input),
        }
    }
}

