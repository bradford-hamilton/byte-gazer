use bg::ByteGazer;
use std::{env, process};

mod bg;
mod colors;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: bytegazer <path_to_file>");
        process::exit(1);
    }

    let file_path = &args[1];
    let byte_gazer = match ByteGazer::new(file_path) {
        Ok(bg) => bg,
        Err(e) => {
            eprintln!("failed to initialize ByteGazer: {}", e);
            process::exit(1);
        }
    };

    byte_gazer.display();
}
