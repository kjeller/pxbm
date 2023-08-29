use pxbm::parser::{netpbm::Netpbm, xbm::Xbm, Parser as TypeParser, xpm::Xpm};

use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use anyhow::Result;

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
    Xpm,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let input = std::fs::read(cli.filepath)?;
    let parser: Box<dyn TypeParser> = match cli.command {
        Command::Xbm => Box::new(Xbm::parse(&input)),
        Command::Netpbm => Box::new(Netpbm::parse(&input)?),
        Command::Xpm => Box::new(Xpm::parse(&input)?),
    };

    parser.print(cli.r, cli.g, cli.b);
    Ok(())
}
