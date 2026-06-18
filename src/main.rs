use alea_jact_est::basic::{
    gen_alphabetic::GenRandomAlphabetic, gen_alphanumeric::GenRandomAlphanumeric,
    gen_numeric::GenRandomNumeric,
};
use alea_jact_est::generator::Generator;
use clap::{Command, Parser, ValueEnum};
use rand::rngs::SysRng;

#[derive(ValueEnum, Clone)]
pub enum Charset {
    #[value(name = "alphanumeric")]
    Alphanumeric,
    #[value(name = "numeric")]
    Numeric,
    #[value(name = "alphabetic")]
    Alphabetic,
}

#[derive(Parser)]
#[command(
    version,
    about = "A fast and secure random string generator CLI",
    long_about = "alea-jact-est generates cryptographically secure random words or strings based on customizable character sets.",
    help_template = "{before-help}{name} {version}\n{about-section}\n{usage-heading} {usage}\n\n{all-args}{after-help}"
)]
struct Args {
    #[arg(
        short = 'l',
        long = "length",
        default_value_t = 5,
        value_name = "NUMBER",
        value_parser = clap::value_parser!(u8).range(1..=255),
        long_help = "Defines how many characters the output word will have. Must be a positive integer up to 255."
    )]
    length: u8,

    #[arg(
        short = 'c',
        long = "charset",
        value_enum,
        default_value_t = Charset::Alphanumeric,
        value_name = "STRATEGY",
        long_help = "Choose the subset of characters used during generation:\n  - alphanumeric: 0-9, A-Z, a-z\n  - numeric: 0-9\n  - alphabetic: A-Z, a-z"
    )]
    charset: Charset,

    #[arg(
        short = 'n',
        long = "count",
        default_value_t = 1,
        value_name = "NUMBER",
        long_help = "The quantity of random strings that will be printed, each on a new line."
    )]
    count: u8,

    #[arg(
        long = "uppercase",
        default_value_t = false,
        long_help = "Restricts the character pool to uppercase letters. Only effective with 'alphanumeric' and 'alphabetic' charsets."
    )]
    uppercase: bool,

    #[arg(
        long = "lowercase",
        default_value_t = false,
        long_help = "Restricts the character pool to lowercase letters. Only effective with 'alphanumeric' and 'alphabetic' charsets."
    )]
    lowercase: bool,
}

fn main() {
    let args = Args::parse();
    let mut rng = SysRng;

    match args.charset {
        Charset::Alphanumeric => {
            let gra = GenRandomAlphanumeric::try_new(args.lowercase, args.uppercase);
            match gra {
                Ok(g) => {
                    std::iter::repeat_n((), args.count as usize).for_each(|_| {
                        println!(
                            "{}",
                            GenRandomAlphanumeric::generate(&g, &mut rng, args.length).unwrap()
                        )
                    });
                }
                Err(e) => {
                    let mut cmd = Command::new("alea-jact-est");
                    cmd.error(clap::error::ErrorKind::TooManyValues, e).exit();
                }
            }
        }
        Charset::Numeric => {
            let grn = GenRandomNumeric;
            std::iter::repeat_n((), args.count as usize).for_each(|_| {
                println!(
                    "{}",
                    GenRandomNumeric::generate(&grn, &mut rng, args.length).unwrap()
                )
            });
        }
        Charset::Alphabetic => {
            let gra = GenRandomAlphabetic::try_new(args.lowercase, args.uppercase);
            match gra {
                Ok(g) => std::iter::repeat_n((), args.count as usize).for_each(|_| {
                    println!(
                        "{}",
                        GenRandomAlphabetic::generate(&g, &mut rng, args.length).unwrap()
                    )
                }),
                Err(e) => {
                    let mut cmd = Command::new("alea-jact-est");
                    cmd.error(clap::error::ErrorKind::TooManyValues, e).exit();
                }
            }
        }
    }
}
