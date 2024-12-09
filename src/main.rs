#[inline]
fn print_help_message() {
    eprintln!("Usage: ");
    eprintln!("\tascii_help <string>       convert the string to ASCII values");
    eprintln!("\tascii_help [numbers...]   convert the numbers to ASCII characters");
    eprintln!("\tascii_help -h             prints this message");
}

fn byte_to_ascii(byte: u8) -> String {
    // 0..=32 + 127
    let special_number = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 
        22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 127
    ];

    let special_repr = [
        "NUL", "SOH", "STX", "ETX", "EOT", "ENQ", "ACK", "BEL", "BS", "HT", "LF", "VT",
        "FF", "CR", "SO", "SI", "DLE", "DC1", "DC2", "DC3", "DC4", "NAK", "SYN", "ETB",
        "CAN", "EM", "SUB", "ESC", "FS", "GS", "RS", "US", "SPACE", "DEL"
    ];

    if byte <= 32 || byte == 127 {
        let idx = special_number.iter().position(|&x| x == byte).unwrap();
        let special_repr = "<".to_string() + special_repr[idx] + ">";
        return special_repr.to_string();
    }

    let c = byte as char;
    c.to_string()
}

fn parse_input_to_array(input: &str) -> Vec<u8> {
    let num_arr = Vec::new();

    for c in input.chars() {
        if c.is_digit(10) {
            let num = c.to_digit(10).unwrap() as u8;
            num_arr.push(num);
        }
    }
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

    dbg!(&str_arg);

    // if the input is of form "[num0, num1, num2, ...]" then convert it to ASCII characters
    if str_arg.starts_with('[') && str_arg.ends_with(']') {
        dbg!("array path");

        let mut ascii_chars = parse_input_to_array(&str_arg);

        let mut ascii_chars = Vec::new();
        let numbers = str_arg[1..str_arg.len()-1].split(", ");
        for num in numbers {
            dbg!(&num);
            let num = num.parse::<u8>().unwrap();
            ascii_chars.push(num);
        }

        let ascii_str: String = ascii_chars.iter().map(|&c| byte_to_ascii(c)).collect();
        println!("{}", ascii_str);

    } else { // else convert the string to ASCII values
        let ascii_values: Vec<u8> = str_arg.chars().map(|c| c as u8).collect();
        println!("{:?}", ascii_values);
    }

}

