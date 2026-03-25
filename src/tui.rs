//! ANSI color and text style utilities.

/// Foreground colors using ANSI escape codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

impl Color {
    fn ansi_code(self) -> u8 {
        match self {
            Color::Black => 30,
            Color::Red => 31,
            Color::Green => 32,
            Color::Yellow => 33,
            Color::Blue => 34,
            Color::Magenta => 35,
            Color::Cyan => 36,
            Color::White => 37,
            Color::BrightBlack => 90,
            Color::BrightRed => 91,
            Color::BrightGreen => 92,
            Color::BrightYellow => 93,
            Color::BrightBlue => 94,
            Color::BrightMagenta => 95,
            Color::BrightCyan => 96,
            Color::BrightWhite => 97,
        }
    }
}

/// Text style modifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Style {
    Bold,
    Dim,
    Italic,
    Underline,
    Strikethrough,
}

impl Style {
    fn ansi_code(self) -> u8 {
        match self {
            Style::Bold => 1,
            Style::Dim => 2,
            Style::Italic => 3,
            Style::Underline => 4,
            Style::Strikethrough => 9,
        }
    }
}

/// Wraps `text` with the ANSI escape sequence for the given foreground `color`.
///
/// # Example
/// ```
/// use termz::tui::{colorize, Color};
/// let s = colorize("hello", Color::Green);
/// println!("{s}");
/// ```
pub fn colorize(text: &str, color: Color) -> String {
    format!("\x1b[{}m{}\x1b[0m", color.ansi_code(), text)
}

/// Wraps `text` with one or more ANSI style codes.
///
/// # Example
/// ```
/// use termz::tui::{stylize, Style};
/// let s = stylize("important", &[Style::Bold, Style::Underline]);
/// println!("{s}");
/// ```
pub fn stylize(text: &str, styles: &[Style]) -> String {
    if styles.is_empty() {
        return text.to_string();
    }
    let codes: Vec<String> = styles.iter().map(|s| s.ansi_code().to_string()).collect();
    format!("\x1b[{}m{}\x1b[0m", codes.join(";"), text)
}

/// Wraps `text` with both a foreground color and style modifiers.
pub fn paint(text: &str, color: Color, styles: &[Style]) -> String {
    let mut codes: Vec<String> = styles.iter().map(|s| s.ansi_code().to_string()).collect();
    codes.insert(0, color.ansi_code().to_string());
    format!("\x1b[{}m{}\x1b[0m", codes.join(";"), text)
}

/// Returns the ANSI reset escape sequence.
pub fn reset() -> &'static str {
    "\x1b[0m"
}

/// Clears the terminal screen.
pub fn clear_screen() -> &'static str {
    "\x1b[2J\x1b[H"
}
