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

    let mut ascii_10 = 0;
    if args.alphabet != "" 
        {
        ascii_10 = args.alphabet.as_bytes()[0];
        }

    let base: u8;
    match args.base
        {
        Some(b) => 
            {
            base = b;
            }
        None =>
            {  
            base = 10;
            }
        }

    match args.alphabet.as_str()
        {
        "" => 
            {
            match args.number
                {
                Some(_) => 
                    {
                    }
                None => 
                    {
                    println!("{}", 0);
                    }
                }
            }
        _ => 
            {
            println!("{}", convert_to_num_wrt_base(ascii_10 as u32, base as u32));
            }
        }

    match args.number
        {
        Some(n) => 
            {
            println!("{}", convert_num_to_char(convert_base_from_x_to_y(base, 10, n)));
            }
        None => 
            {
            }
        }
    }

fn convert_to_num_wrt_base(num: u32, to_base: u32) -> String
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

fn convert_num_to_char(num: u8) -> String
    {
    let byte_arr = [num];
    String::from_utf8_lossy(&byte_arr).to_string()
    }

fn convert_base_from_x_to_y(before_base: u8, _after_base: u8, num: u8) -> u8
    {
    // TODO: convert a given number num in base x to base y
    // return the new number
    match before_base
        {
        16 =>
            {
            let num_str = num.to_string();
            u64::from_str_radix(&String::from(num_str), 16).unwrap() as u8
            }
        10 => 
            {
            num
            }
        8 => 
            {
            u64::from_str_radix(&String::from(num.to_string()), 8).unwrap() as u8
            }
        _ => 
            {
            panic!("Base not supported {}", before_base);
            }
        }
    }
