pub enum Foreground {
    Black = 30,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightWhite,
}

pub enum Background {
    Black = 40,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightWhite,
}

pub fn apply_to_text(text: &str, foreground: Foreground, background: Background) -> String {
    format!(
        "\x1B[{};{}m{}\x1B[0m",
        foreground as i32, background as i32, text
    )
}
