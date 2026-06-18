use rand::TryRng;
use thiserror::Error;

use crate::generator::{Generator, GeneratorError};

const ALPHANUMERIC_CHARSET: &[u8] =
    b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

const UPPERCASE_ALPHANUMERIC_CHARSET: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

const LOWERCASE_ALPHANUMERIC_CHARSET: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyz";

enum Charset {
    Uppercase,
    Lowercase,
    Whatever,
}

#[derive(Error, Debug)]
pub enum GenRandomAlphanumericError {
    #[error("lowercase and uppercase cannot be true at the same time")]
    InvalidArgs,
}

pub struct GenRandomAlphanumeric {
    charset: Charset,
}

impl GenRandomAlphanumeric {
    pub fn try_new(lowercase: bool, uppercase: bool) -> Result<Self, GenRandomAlphanumericError> {
        if lowercase && uppercase {
            Err(GenRandomAlphanumericError::InvalidArgs)
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

impl Default for GenRandomAlphanumeric {
    fn default() -> Self {
        Self {
            charset: Charset::Whatever,
        }
    }
}

impl GenRandomAlphanumeric {
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

impl Generator for GenRandomAlphanumeric {
    fn generate<R>(&self, try_rng: &mut R, length: u8) -> Result<String, GeneratorError>
    where
        GeneratorError: From<R::Error>,
        R: TryRng,
    {
        match self.charset {
            Charset::Uppercase => GenRandomAlphanumeric::generate_word(
                UPPERCASE_ALPHANUMERIC_CHARSET,
                try_rng,
                length,
            ),
            Charset::Lowercase => GenRandomAlphanumeric::generate_word(
                LOWERCASE_ALPHANUMERIC_CHARSET,
                try_rng,
                length,
            ),
            Charset::Whatever => {
                GenRandomAlphanumeric::generate_word(ALPHANUMERIC_CHARSET, try_rng, length)
            }
        }
    }
}
