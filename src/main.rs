use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

//
//
// Looking at this code will reveal the easter egg
//
//

fn encode_to_hex(string_data: &str) -> String {
    string_data
        .as_bytes()
        .iter()
        .map(|byte| format!("{:02X}", byte))
        .collect::<Vec<_>>()
        .join(" ")
}

fn decode_from_hex(hex_representation: &str) -> Result<String, String> {
    // Expect space-separated hex bytes like "48 65 6C 6C 6F"
    let mut bytes = Vec::new();
    for chunk in hex_representation.split_whitespace() {
        if chunk.len() != 2 {
            return Err(format!("Invalid hex chunk length (need 2): {}", chunk));
        }
        match u8::from_str_radix(chunk, 16) {
            Ok(byte) => bytes.push(byte),
            Err(_) => return Err(format!("Invalid hex chunk: {}", chunk)),
        }
    }
    String::from_utf8(bytes).map_err(|e| format!("Invalid UTF-8 sequence: {}", e))
}

fn read_binary_file(file_path: &PathBuf) -> Option<Vec<u8>> {
    match fs::read(file_path) {
        Ok(data) => Some(data),
        Err(err) => {
            println!("File not found: {}", err);
            None
        }
    }
}

fn write_binary_file(file_path: &PathBuf, data: &[u8]) {
    if let Some(parent) = file_path.parent() {
        if let Err(err) = fs::create_dir_all(parent) {
            println!("An error occurred: {}", err);
            return;
        }
    }

    match fs::write(file_path, data) {
        Ok(_) => println!("Data successfully written to file: {:?}", file_path),
        Err(err) => println!("An error occurred: {}", err),
    }
}

fn binary_to_string(data: &[u8]) -> String {
    data.iter()
        .map(|byte| format!("{:08b}", byte))
        .collect::<Vec<_>>()
        .join("")
}

fn string_to_binary(data_string: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    for chunk in data_string.as_bytes().chunks(8) {
        let byte = chunk
            .iter()
            .map(|bit| (bit != &b'0') as u8)
            .fold(0u8, |acc, bit| (acc << 1) | bit);
        bytes.push(byte);
    }
    bytes
}

fn upside_down_text(text: &str) -> String {
    let upside_down_chars = [
        ('a', 'ɒ'), ('b', 'q'), ('c', 'ɔ'), ('d', 'p'), ('e', 'ǝ'), ('f', 'ɟ'), ('g', 'ƃ'),
        ('h', 'ɥ'), ('i', 'ᴉ'), ('j', 'ɾ'), ('k', 'ʞ'), ('l', 'l'), ('m', 'ɯ'), ('n', 'u'),
        ('o', 'o'), ('p', 'd'), ('q', 'b'), ('r', 'ɹ'), ('s', 's'), ('t', 'ʇ'), ('u', 'n'),
        ('v', 'ʌ'), ('w', 'ʍ'), ('x', 'x'), ('y', 'ʎ'), ('z', 'z'), ('A', '∀'), ('B', 'Ɓ'),
        ('C', 'Ɔ'), ('D', 'ᗡ'), ('E', 'Ǝ'), ('F', 'Ⅎ'), ('G', '⅁'), ('H', 'H'), ('I', 'I'),
        ('J', 'ſ'), ('K', 'ʞ'), ('L', '˥'), ('M', 'W'), ('N', 'N'), ('O', 'O'), ('P', 'Ԁ'),
        ('Q', 'Ό'), ('R', 'ᴚ'), ('S', 'S'), ('T', '⊥'), ('U', '∩'), ('V', 'Λ'), ('W', 'M'),
        ('X', 'X'), ('Y', '⅄'), ('Z', 'Z'), ('0', '0'), ('1', 'Ɩ'), ('2', 'ᄅ'), ('3', 'Ɛ'),
        ('4', 'ᔭ'), ('5', 'S'), ('6', '9'), ('7', 'ㄣ'), ('8', '8'), ('9', '6'), (',', '’'),
        ('.', '˙'), ('?', '¿'), ('!', '¡'), ('\'', ','), ('"', ','), ('(', ')'), (')', '('),
        ('[', ']'), (']', '['), ('{', '}'), ('}', '{'), ('<', '>'), ('>', '<'), ('&', '⅋'),
        ('_', '‾'), ('/', '\\'), ('\\', '/'), (';', '؛'), (':', ':'), ('@', '@'), ('#', '#'),
        ('$', '$'), ('%', '%'), ('^', 'v'), ('*', '*'), ('-', '-'), ('+', '+'), ('=', '='),
        ('~', '~'), ('`', '˙'), (' ', ' '),
    ];

    let mut upside_down_text = String::new();
    for ch in text.chars() {
        if let Some((_, ud)) = upside_down_chars.iter().find(|(c, _)| *c == ch) {
            upside_down_text.push(*ud);
        } else {
            upside_down_text.push(ch);
        }
    }
    upside_down_text.chars().rev().collect()
}

fn run_hex_script() {
    loop {
        println!("1. Encode string to hex");
        println!("2. Decode hex to string");
        println!("3. Exit");

        let mut choice = String::new();
        if io::stdin().read_line(&mut choice).is_err() {
            eprintln!("Failed to read line");
            continue;
        }

        match choice.trim() {
            "1" => {
                println!("Enter the string to encode:");
                let mut original_data = String::new();
                if io::stdin().read_line(&mut original_data).is_err() {
                    eprintln!("Failed to read line");
                    continue;
                }
                println!("Encoded hex: {}", encode_to_hex(original_data.trim()));
            }
            "2" => {
                println!("Enter hex data to decode (e.g., XX XX XX XX, with spaces):");
                let mut hex_value = String::new();
                if io::stdin().read_line(&mut hex_value).is_err() {
                    eprintln!("Failed to read line");
                    continue;
                }
                match decode_from_hex(hex_value.trim()) {
                    Ok(s) => println!("Decoded data: {}", s),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            "3" => break,
            _ => println!("Invalid choice. Please enter 1, 2, or 3."),
        }
    }

    println!("Press Enter to exit...");
    let _ = io::stdin().read_line(&mut String::new());
}

fn run_tobab_script() {
    println!("1. File to binary\n2. Binary to file\nType 1 or 2 and press Enter:");

    let mut choice = String::new();
    if io::stdin().read_line(&mut choice).is_err() {
        eprintln!("Failed to read line");
        return;
    }

    match choice.trim() {
        "1" => {
            println!("Enter the file path (without quotes):");
            let mut file_path = String::new();
            if io::stdin().read_line(&mut file_path).is_err() {
                eprintln!("Failed to read line");
                return;
            }
            let file_path = PathBuf::from(file_path.trim());

            if let Some(data) = read_binary_file(&file_path) {
                println!("Data as a binary string:");
                println!("{}", binary_to_string(&data));
            }
        }
        "2" => {
            println!("Enter the destination folder path (without quotes):");
            let mut folder_path = String::new();
            if io::stdin().read_line(&mut folder_path).is_err() {
                eprintln!("Failed to read line");
                return;
            }
            let folder_path = PathBuf::from(folder_path.trim());

            println!("Enter the file name with extension:");
            let mut file_name = String::new();
            if io::stdin().read_line(&mut file_name).is_err() {
                eprintln!("Failed to read line");
                return;
            }

            println!("Enter the binary string:");
            let mut binary_string = String::new();
            if io::stdin().read_line(&mut binary_string).is_err() {
                eprintln!("Failed to read line");
                return;
            }

            let data = string_to_binary(binary_string.trim());
            let file_path = folder_path.join(file_name.trim());
            write_binary_file(&file_path, &data);
        }
        _ => println!("Invalid input. Please enter either '1' or '2'."),
    }
}

fn mainmenu() {
    loop {
        println!("1. Hexadecimal En/Decoder");
        println!("2. File to Binary and Back (only works with small files right now)");
        println!("3. Upside-down Text");
        println!("4. Exit");
        print!("Please choose an option between 1 and 4: ");
        let _ = io::stdout().flush();

        let mut choice = String::new();
        if io::stdin().read_line(&mut choice).is_err() {
            eprintln!("Failed to read line");
            continue;
        }
        let choice = choice.trim();

        match choice.to_ascii_lowercase().as_str() {
            "1" => run_hex_script(),
            "2" => run_tobab_script(),
            "3" => {
                println!("Enter the text you want to turn upside down:");
                let mut original_text = String::new();
                if io::stdin().read_line(&mut original_text).is_err() {
                    eprintln!("Failed to read line");
                    continue;
                }
                let upside_down_result = upside_down_text(original_text.trim());
                println!("Original text: {}", original_text.trim());
                println!("Upside-down text: {}", upside_down_result);
                println!("Press Enter to close the window...");
                let mut _pause = String::new();
                let _ = io::stdin().read_line(&mut _pause);
            }
            "4" => {
                println!("Closing.");
                break;
            }

            // Easter egg: numeric code or hint word
            "7355608" => {
                println!("The Bomb Has Been Planted");
                continue;
            }

            // Fallback: numeric parse
            other => match other.parse::<u32>() {
                Ok(n @ 1..=4) => match n {
                    1 => run_hex_script(),
                    2 => run_tobab_script(),
                    3 => {
                        println!("Enter the text you want to turn upside down:");
                        let mut original_text = String::new();
                        if io::stdin().read_line(&mut original_text).is_err() {
                            eprintln!("Failed to read line");
                            continue;
                        }
                        let upside_down_result = upside_down_text(original_text.trim());
                        println!("Original text: {}", original_text.trim());
                        println!("Upside-down text: {}", upside_down_result);
                        println!("Press Enter to close the window...");
                        let mut _pause = String::new();
                        let _ = io::stdin().read_line(&mut _pause);
                    }
                    4 => {
                        println!("Closing.");
                        break;
                    }
                    _ => unreachable!(),
                },
                _ => {
                    println!("Invalid selection. Please select an option between 1 and 4. (Unless you're trying to find the Easter egg.)");
                }
            },
        }
    }
}

// csbc = counter-strike bomb code

fn main() {
    mainmenu();
}
