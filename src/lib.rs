pub mod parser;
pub mod util;
pub mod color;


#[derive(Debug, thiserror::Error)]
pub enum PxbmError {
    #[error("Input file contains invalid characters.")]
    BadUtf8,

    #[error("Encountered unexpected end of file.")]
    Eof
}