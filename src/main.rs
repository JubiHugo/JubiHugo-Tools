use std::env;
use std::fs;
use std::io::{self, Read, Write};
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
    if hex_representation.len() % 2 != 0 {
        return Err("Hex input must have an even number of characters.".to_string());
    }
    let mut bytes = Vec::new();
    for chunk in hex_representation.split_whitespace() {
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
            println!("Die Datei wurde nicht gefunden: {}", err);
            None
        }
    }
}

fn write_binary_file(file_path: &PathBuf, data: &[u8]) {
    if let Some(parent) = file_path.parent() {
        if let Err(err) = fs::create_dir_all(parent) {
            println!("Ein Fehler ist aufgetreten: {}", err);
            return;
        }
    }

    match fs::write(file_path, data) {
        Ok(_) => println!("Die Daten wurden erfolgreich in die Datei geschrieben: {:?}", file_path),
        Err(err) => println!("Ein Fehler ist aufgetreten: {}", err),
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
    for char in text.chars() {
        if let Some((_, upside_down_char)) = upside_down_chars.iter().find(|(c, _)| *c == char) {
            upside_down_text.push(*upside_down_char);
        } else {
            upside_down_text.push(char);
        }
    }
    upside_down_text.chars().rev().collect()
}

fn run_hex_script() {
    loop {
        println!("1. Encode String to Hex");
        println!("2. Decode Hex to String");
        println!("3. Exit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim() {
            "1" => {
                println!("Enter the string to encode:");
                let mut original_data = String::new();
                io::stdin()
                    .read_line(&mut original_data)
                    .expect("Failed to read line");
                println!(
                    "Encoded Hex: {}",
                    encode_to_hex(&original_data.trim())
                );
            }
            "2" => {
                println!("Enter hexadecimal data to decode (e.g., XX XX XX XX and dont forget the spaces):");
                let mut hex_value = String::new();
                io::stdin()
                    .read_line(&mut hex_value)
                    .expect("Failed to read line");
                println!("Decoded Data: {}", decode_from_hex(&hex_value.trim()));
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
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    match choice.trim() {
        "1" => {
            println!("Please enter the file path (without quotes):");
            let mut file_path = String::new();
            io::stdin()
                .read_line(&mut file_path)
                .expect("Failed to read line");
            let file_path = PathBuf::from(file_path.trim());

            if let Some(data) = read_binary_file(&file_path) {
                println!("Data in binary form as a string:");
                println!("{}", binary_to_string(&data));
            }
        }
        "2" => {
            println!("Enter the path to a folder where the file should end up (without quotes):");
            let mut folder_path = String::new();
            io::stdin()
                .read_line(&mut folder_path)
                .expect("Failed to read line");
            let folder_path = PathBuf::from(folder_path.trim());

            println!("What should the file-name be with attachment:");
            let mut file_name = String::new();
            io::stdin()
                .read_line(&mut file_name)
                .expect("Failed to read line");

            println!("Please enter the binary string:");
            let mut binary_string = String::new();
            io::stdin()
                .read_line(&mut binary_string)
                .expect("Failed to read line");

            let data = string_to_binary(&binary_string.trim());
            let file_path = folder_path.join(file_name.trim());
            write_binary_file(&file_path, &data);
        }
        _ => println!("Invalid input. Please enter either '1' or '2'."),
    }
}

fn mainmenu() {
    loop {
        println!("1. Hexadecimal En/Decoder");
        println!("2. File to Binary and Back (only works with small files rn)");
        println!("3. Upsidedown Text");
        println!("4. Exit");

        println!("Please choose an option between 1 and 4:");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim().parse::<u32>() {
            Ok(1) => run_hex_script(),
            Ok(2) => run_tobab_script(),
            Ok(3) => {
                println!("Enter the text you want to turn upside down:");
                let mut original_text = String::new();
                io::stdin()
                    .read_line(&mut original_text)
                    .expect("Failed to read line");
                let upside_down_result = upside_down_text(&original_text.trim());
                println!("Original Text: {}", original_text.trim());
                println!("Upsidedown Text: {}", upside_down_result);
                println!("Press enter to clse the window...");
                let _ = io::stdin().read_line(&mut String::new());
            }
            Ok(4) => {
                println!("Closing.");
                break;
            }
            Ok(7355608) => {println!("The Bomb Has Been Planted"); break}
            Ok(code) => {println!("7355608")}
            _ => println!("Invalid selection. Please select an option between 1 to 4 or the csbc."),
        }
    }
}
// csbc = counter-strike bomb code
fn main() {
    mainmenu();
}