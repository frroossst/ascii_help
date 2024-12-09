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
    let mut num_arr = Vec::new();

    let input = input.trim();

    // if there are no commas then return the singular number
    if !input.contains(",") {
        let num = input[1..input.len()-1].parse::<u8>().unwrap();
        num_arr.push(num);
        return num_arr;
    }

    // else split the string by commas and parse each number
    let numbers = input[1..input.len()-1].split(",");
    for num in numbers {
        let num = num.trim().parse::<u8>().unwrap();
        num_arr.push(num);
    }
    num_arr
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        print_help_message();
        std::process::exit(1);
    }

    if args.len() == 2 && (args[1] == "-h" || args[1] == "--help") {
        print_help_message();
        std::process::exit(0);
    }

    let str_arg = &args[1..].join(" ");
    let stripped = str_arg.trim();

    // if the input is of form "[num0, num1, num2, ...]" then convert it to ASCII characters
    if str_arg.starts_with('[') && stripped.ends_with(']') {
        let ascii_chars = parse_input_to_array(&str_arg);

        let ascii_str: String = ascii_chars.iter().map(|&c| byte_to_ascii(c)).collect();
        println!("{}", ascii_str);

    } else { // else convert the string to ASCII values
        let ascii_values: Vec<u8> = str_arg.chars().map(|c| c as u8).collect();
        println!("{:?}", ascii_values);
    }

}

