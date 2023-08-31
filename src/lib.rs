pub mod parser;
pub mod util;
pub mod color;


#[derive(Debug, thiserror::Error)]
pub enum PxbmError {
    #[error("Input file contains invalid characters.")]
    BadUtf8,

    #[error("Failed to write to output stream.")]
    WriteFailure,

    #[error("Encountered unexpected end of file.")]
    Eof
}

#[macro_export]
macro_rules! pxbm_write {
    ($writer:ident, $fmt:literal $(, $fmt_arg:expr),*) => {
        write!($writer, $fmt$(, $fmt_arg)*).map_err(|_| $crate::PxbmError::WriteFailure)
    };
}

#[macro_export]
macro_rules! pxbm_writeln {
    ($writer:ident $(, $fmt:literal)? $(, $fmt_arg:expr),*) => {
        writeln!($writer$(, $fmt)?$(, $fmt_arg)*).map_err(|_| $crate::PxbmError::WriteFailure)
    };
}