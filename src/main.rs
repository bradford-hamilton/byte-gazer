use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let path = "test.hex";
    let bytes = read_to_binary(path)?;

    print_hex_view(&bytes);

    Ok(())
}

fn read_to_binary(path: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;

    Ok(buffer)
}

fn to_hex_string(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<Vec<String>>()
        .join(" ")
}

fn print_hex_view(bytes: &[u8]) {
    for chunk in bytes.chunks(16) {
        let hex_part = to_hex_string(chunk);
        let ascii_part = chunk
            .iter()
            .map(|&b| {
                if b.is_ascii_alphanumeric() || b.is_ascii_punctuation() || b.is_ascii_whitespace()
                {
                    b as char
                } else {
                    '.'
                }
            })
            .collect::<String>();

        println!("{}  {}", hex_part, ascii_part);
    }
}
