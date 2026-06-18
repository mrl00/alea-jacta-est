use rand::TryRng;
use thiserror::Error;

use crate::generator::{Generator, GeneratorError};

const NUMERIC_CHARSET: &[u8] = b"0123456789";

#[derive(Error, Debug)]
pub enum GenRandomNumericError {
    #[error("lowercase and uppercase flags are not supported for numeric charset")]
    UnsupportedArgs,
}

pub struct GenRandomNumeric;

impl GenRandomNumeric {
    fn generate_word<R>(
        charset: &[u8],
        try_rng: &mut R,
        length: u8,
    ) -> Result<String, GeneratorError>
    where
        GeneratorError: From<R::Error>,
        R: TryRng,
    {
        let k: Result<Vec<u8>, _> = (0..length)
            .map(|_| {
                let random_val = try_rng.try_next_u32()?;
                let index = random_val as usize % charset.len();
                Ok(charset[index])
            })
            .collect();

        let bytes = k?;

        let result_string = String::from_utf8(bytes)?;

        Ok(result_string)
    }
}

impl Generator for GenRandomNumeric {
    fn generate<R>(&self, try_rng: &mut R, length: u8) -> Result<String, GeneratorError>
    where
        GeneratorError: From<R::Error>,
        R: TryRng,
    {
        GenRandomNumeric::generate_word(NUMERIC_CHARSET, try_rng, length)
    }
}
