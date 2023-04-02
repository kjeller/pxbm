mod parser;
mod util;

use parser::Format;
use parser::FormatParser;
use parser::PrintableFormat;

fn main() {
    // TODO fix cli arguments
    let fp = FormatParser{ format: Format::Xbm };
    let input = util::to_string("res/xbm/loink.xbm");
    let p = fp.parse(input.as_str());
    p.print(255, 0, 0);
}
