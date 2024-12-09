#[inline]
fn print_help_message() {
    eprintln!("Usage: ");
    eprintln!("\tascii_help <string>       convert the string to ASCII values");
    eprintln!("\tascii_help [numbers...]   convert the numbers to ASCII characters");
    eprintln!("\tascii_help -h             prints this message");
}

fn main() {
    let mut args = std::env::args();
    let _program = args.next().unwrap();

    let mut str_arg = match args.next() {
        Some(s) => { 
            // say <NON-ASCII> if any of the characters are not ASCII
            let is_ascii = s.chars().all(|c| c.is_ascii());
            if is_ascii {
                s
            } else {
                eprintln!("<NON-ASCII>");
                std::process::exit(1);
            }
        },
        None => {
            print_help_message();
            std::process::exit(1);
        },
    };

    // since the input may be space seperated collect all the arguments
    let rem_str_args = args.collect::<Vec<String>>();
    str_arg = str_arg + " " + &rem_str_args.join(" ");

    // if the input is of form "[num0, num1, num2, ...]" then convert it to ASCII characters
    if str_arg.starts_with('[') && str_arg.ends_with(']') {
        let mut ascii_chars = Vec::new();
        let numbers = str_arg[1..str_arg.len()-1].split(", ");
        for num in numbers {
            let num = num.parse::<u8>().unwrap();
            let c = num as char;
            ascii_chars.push(c);
        }
        let ascii_str: String = ascii_chars.into_iter().collect();
        println!("{}", ascii_str);
    } else { // else convert the string to ASCII values
        let ascii_values: Vec<u8> = str_arg.chars().map(|c| c as u8).collect();
        println!("{:?}", ascii_values);
    }

}

