use core::fmt;

use crate::util::PointInfo;
use core::error::Error as CoreError;
use thiserror::Error;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpannedError<E> {
    pub error: E,
    pub at: PointInfo,
}
impl<E> SpannedError<E> {
    pub fn new(error: E, at: PointInfo) -> Self {
        return Self { error, at };
    }
}
impl<T> fmt::Display for SpannedError<T>
where
    T: fmt::Display + CoreError,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut err: &dyn CoreError = &self.error;
        write!(f, "{}", err)?; // top-level error
        while let Some(source) = err.source() {
            write!(f, "\n→ caused by: {}", source)?;
            err = source;
        }
        return write!(
            f,
            "\n\n[ at line {}, column {} ]",
            self.at.line, self.at.column
        );
    }
}

// impl<T> CoreError for SpannedError<T>
// where
//     T: CoreError + 'static,
// {
//     fn source(&self) -> Option<&(dyn CoreError + 'static)> {
//         return self.error.source();
//     }
// }
#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnicodeEscapeError {
    #[error("Missing opening '{{' in Unicode escape")]
    MissingOpeningBrace,

    #[error("Missing closing '}}' in Unicode escape")]
    MissingClosingBrace,

    #[error("Invalid hex digits in Unicode escape")]
    InvalidHex,
}
#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringError {
    #[error("Unterminated string literal")]
    UnterminatedStringLiteral,
    #[error("Unknown escape: \\{0}")]
    UnknownEscape(char),
    #[error(transparent)]
    UnicodeEscape(#[from] UnicodeEscapeError),
}
#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumberError {
    #[error("Too many dots for number")]
    TooManyDots
}
#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeError {
    // since mostly for this, the usize or sum shit are panicking (since theres no way to make it as result)
    // we're gonna add it as .expect()
    // #[error(
    //     "{FAILED_CONVERTING_TO_INDEPENDENT_BITS}"
    // )]
    // FailedConvertingToIndependentBits,
}
#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
#[error("An error occurred during lexing (see stack)")]
pub enum LexError {
    String(#[source] StringError),
    Number(#[source] NumberError),
    #[error("An error occurred during lexing: Unexpected End of content")]
    UnexpectedEof,
    #[error("An error occurred during lexing: Unexpected character")]
    UnexpectedChar,
    #[error(transparent)]
    Runtime(#[from] RuntimeError),
}
