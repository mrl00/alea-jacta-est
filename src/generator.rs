use std::string::FromUtf8Error;

use rand::{TryRng, rngs::SysError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GeneratorError {
    #[error("Falha ao obter bytes aleatórios do sistema: {0}")]
    RngError(#[from] SysError),

    #[error("Generated string is not valid UTF-8: {0}")]
    InvalidUtf8(#[from] FromUtf8Error),
}

pub trait Generator {
    fn generate<R>(&self, try_rng: &mut R, length: u8) -> Result<String, GeneratorError>
    where
        GeneratorError: From<R::Error>,
        R: TryRng;
}
