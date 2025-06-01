#[derive(Copy, Clone)]
pub enum Color {
    Black,
    Blue,
    Brown,
    LightMagenta,
    LightRed,
    Yellow,
    LightGray,
    Red,
    LightGreen,
    LightCyan,
    White,
    LightBlue,
    Green,
    Magenta,
    Cyan,
    DarkGray,
    LightYellow, // Add more colors as needed
}
impl Color {
    pub fn to_ansi_code(&self) -> &str {
        match self {
            Color::Black => "\x1b[40m  \x1b[0m",
            Color::Red => "\x1b[41m  \x1b[0m",
            Color::Green => "\x1b[42m  \x1b[0m",
            Color::Yellow => "\x1b[43m  \x1b[0m",
            Color::Blue => "\x1b[44m  \x1b[0m",
            Color::Magenta => "\x1b[45m  \x1b[0m",
            Color::Cyan => "\x1b[46m  \x1b[0m",
            Color::LightGray => "\x1b[47m  \x1b[0m",
            Color::DarkGray => "\x1b[100m  \x1b[0m",
            Color::LightRed => "\x1b[101m  \x1b[0m",
            Color::LightGreen => "\x1b[102m  \x1b[0m",
            Color::LightYellow => "\x1b[103m  \x1b[0m",
            Color::LightBlue => "\x1b[104m  \x1b[0m",
            Color::LightMagenta => "\x1b[105m  \x1b[0m",
            Color::LightCyan => "\x1b[106m  \x1b[0m",
            Color::White => "\x1b[107m  \x1b[0m",
            Color::Brown => "\x1b[43m  \x1b[0m",
        }
    }
}
