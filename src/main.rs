mod parser;
mod util;

use parser::{netpbm::Netpbm, xbm::Xbm, Parser as TypeParser};

use std::path::PathBuf;

use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(value_name = "command", value_enum)]
    command: Command,

    /// File to print
    #[arg(value_name = "filepath")]
    filepath: PathBuf,

    /// Red
    #[arg(value_name = "red", default_value_t = 255)]
    r: u8,
    /// Green
    #[arg(value_name = "green", default_value_t = 255)]
    g: u8,
    /// Blue
    #[arg(value_name = "blue", default_value_t = 255)]
    b: u8,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Command {
    Xbm,
    Netpbm,
}

fn main() {
    let cli = Cli::parse();
    let input = util::to_string(cli.filepath.as_os_str());
    let parser: Box<dyn TypeParser>;

    match cli.command {
        Command::Xbm => {
            parser = Box::new(Xbm::parse(input.as_str()));
        }
        Command::Netpbm => {
            parser = Box::new(Netpbm::parse(input.as_str()));
        }
    }

    parser.print(cli.r, cli.g, cli.b);
}
