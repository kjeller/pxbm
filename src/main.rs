mod parser;
mod util;

use parser::Format;
use parser::FormatParser;
use parser::PrintableFormat;

fn main() {
    let fp = FormatParser{ format: Format::Xbm };
    let input = util::to_string("res/xbm/loink.xbm");
    let p = fp.parse(input.as_str());
    p.print();
}
