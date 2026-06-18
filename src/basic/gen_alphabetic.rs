use rand::TryRng;
use thiserror::Error;

use crate::generator::{Generator, GeneratorError};

const ALPHABETIC_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

const UPPERCASE_ALPHABETIC_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";

const LOWERCASE_ALPHABETIC_CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";

enum Charset {
    Uppercase,
    Lowercase,
    Whatever,
}

#[derive(Error, Debug)]
pub enum GenRandomAlphabeticError {
    #[error("lowercase and uppercase cannot be true at the same time")]
    InvalidArgs,
}

pub struct GenRandomAlphabetic {
    charset: Charset,
}

impl GenRandomAlphabetic {
    pub fn try_new(lowercase: bool, uppercase: bool) -> Result<Self, GenRandomAlphabeticError> {
        if lowercase && uppercase {
            Err(GenRandomAlphabeticError::InvalidArgs)
        } else if lowercase {
            Ok(Self {
                charset: Charset::Lowercase,
            })
        } else if uppercase {
            Ok(Self {
                charset: Charset::Uppercase,
            })
        } else {
            Ok(Self {
                charset: Charset::Whatever,
            })
        }
    }
}

impl Default for GenRandomAlphabetic {
    fn default() -> Self {
        Self {
            charset: Charset::Whatever,
        }
    }
}

impl GenRandomAlphabetic {
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

impl Generator for GenRandomAlphabetic {
    fn generate<R>(&self, try_rng: &mut R, length: u8) -> Result<String, GeneratorError>
    where
        GeneratorError: From<R::Error>,
        R: TryRng,
    {
        match self.charset {
            Charset::Uppercase => {
                GenRandomAlphabetic::generate_word(UPPERCASE_ALPHABETIC_CHARSET, try_rng, length)
            }
            Charset::Lowercase => {
                GenRandomAlphabetic::generate_word(LOWERCASE_ALPHABETIC_CHARSET, try_rng, length)
            }
            Charset::Whatever => {
                GenRandomAlphabetic::generate_word(ALPHABETIC_CHARSET, try_rng, length)
            }
        }
    }
}
