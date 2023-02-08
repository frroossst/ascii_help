use clap::Parser;


#[derive(Parser, Debug)]
struct Args
    {
    #[clap(value_parser, default_value = "")]
    alphabet: String,

    #[clap(value_parser, short, long)]
    base: Option<u8>,

    #[clap(value_parser, short, long)]
    number: Option<u8>,
    }

fn main() 
    {
    let args = Args::parse();

    let ascii_10 = args.alphabet.as_bytes()[0];

    match args.base
        {
        Some(base) => println!("{}", convert_to_base(ascii_10 as u32, base as u32)),
        None => { println!("{}", ascii_10) },
        }
    }

fn convert_to_base(num: u32, to_base: u32) -> String
    {
    let mut result = String::new();
    let mut n = num;
    while n > 0 
        {
        result.push_str(&((n % to_base) as u8).to_string());
        n /= to_base;
        }
    result.chars().rev().collect::<String>()
    }

fn convert_to_char(num: u32) -> String
    {
    let c = char::from_digit(num, 10).unwrap();
    c.to_string()
    }
