use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid FEN String")]
    InvalidFen,
    #[error("Parse error")]
    ParseError,
}
