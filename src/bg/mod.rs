use std::fs::File;
use std::io::{self, Read};

use crate::colors;

pub struct ByteGazer {
    bytes: Vec<u8>,
}

impl ByteGazer {
    pub fn new(path_to_file: &str) -> io::Result<Self> {
        let mut file = File::open(path_to_file)?;
        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer)?;

        Ok(ByteGazer { bytes: buffer })
    }

    pub fn display(self) {
        let bytes_per_line = 16;

        for chunk in self.bytes.chunks(bytes_per_line) {
            let hex_portion = chunk
                .iter()
                .map(|byte| {
                    colors::apply_to_text(
                        &format!("{:02x} ", byte),
                        colors::Foreground::Blue,
                        colors::Background::Black,
                    )
                })
                .collect::<String>();

            let ascii_portion = chunk
                .iter()
                .map(|&byte| {
                    if byte.is_ascii_graphic() || byte == b' ' {
                        colors::apply_to_text(
                            &format!("{}", byte as char),
                            colors::Foreground::Green,
                            colors::Background::Black,
                        )
                    } else {
                        colors::apply_to_text(
                            ".",
                            colors::Foreground::Red,
                            colors::Background::Black,
                        )
                    }
                })
                .collect::<String>();

            let padding = "   ".repeat(bytes_per_line - chunk.len());

            println!("{}{} | {}", hex_portion, padding, ascii_portion);
        }
    }
}
