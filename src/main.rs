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

fn print_hex_view(bytes: &[u8]) {
    let bytes_per_line = 16;

    for chunk in bytes.chunks(bytes_per_line) {
        let hex_part = chunk
            .iter()
            .map(|byte| format!("{:02x} ", byte))
            .collect::<String>();

        let ascii_part = chunk
            .iter()
            .map(|&b| {
                if b.is_ascii_graphic() || b == b' ' {
                    b as char
                } else {
                    '.'
                }
            })
            .collect::<String>();

        // Calculate padding, ensuring no overflow
        let padding = "   ".repeat(bytes_per_line - chunk.len());

        println!("{}{} | {}", hex_part, padding, ascii_part);
    }
}
