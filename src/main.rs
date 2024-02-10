use bg::ByteGazer;

mod bg;
mod colors;

fn main() {
    // let path = "test-hex-program";
    let path = "test.hex";
    let byte_gazer = match ByteGazer::new(path) {
        Ok(bg) => bg,
        Err(e) => {
            eprintln!("Failed to create ByteGazer: {}", e);
            return;
        }
    };

    byte_gazer.display();
}
