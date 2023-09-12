use std::fs::File;
use std::io::Read;
extern crate hex;

const FILE_NAME: &str = "binary";

fn read_file(path: &str) -> String {
    let mut file = File::open(path).expect("File not found");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Reading error");
    hex::encode(buffer)
}

fn hex_to_ascii(hex_string: &str) -> String {
    let mut ascii_string = String::new();
    let mut chars = hex_string.chars();

    while let Some(c1) = chars.next() {
        if let Some(c2) = chars.next() {
            if let Ok(hex_value) = u8::from_str_radix(&format!("{}{}", c1, c2), 16) {
                if hex_value.is_ascii() {
                    ascii_string.push(hex_value as char);
                }
            }
        }
    }

    ascii_string
}

fn format(hex_string: &str) -> String {
    let mut spaced_string = String::new();
    let mut chars = hex_string.chars();
    let mut count = 0;
    let mut lines = 0;

    while let Some(c1) = chars.next() {
        if let Some(c2) = chars.next() {
            // start of line
            if count % 32 == 0 {
                spaced_string += &format!("{:0width$X}", lines * 16, width = 8);
                spaced_string += "  ";
            }
            spaced_string.push(c1);
            spaced_string.push(c2);
            count += 2;

            // after 8 bytes
            if count % 16 == 0 && count % 32 != 0 {
                spaced_string += " ";
            }

            // end of line
            if count % 32 == 0 {
                let sliced = &hex_string[lines * 32..(lines + 1) * 32];
                spaced_string += "  |";
                spaced_string += &hex_to_ascii(sliced);
                spaced_string += "|";
                spaced_string.push('\n'); // Add a newline every 32 characters
                lines += 1;
            } else {
                spaced_string.push(' '); // Add a space between every two characters
            }
        }
    }

    // check if we are on the last line
    if count % 32 != 0 {
        let line_length = 48;
        let sliced = &hex_string[lines * 32..];
        let remaining = line_length - (sliced.len() + (sliced.len() / 2));

        // Add spaces to align the last line
        spaced_string += &" ".repeat(remaining);

        spaced_string += "  |";
        spaced_string += &hex_to_ascii(sliced).trim_end();
        spaced_string += "|";
    }

    // Remove the trailing space, if any
    if spaced_string.ends_with(' ') {
        spaced_string.pop();
    }

    spaced_string
}

fn main() {
    let hex_string = read_file(FILE_NAME);
    println!("{}", format(&hex_string));
}
