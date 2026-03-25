//! General string and value manipulation helpers.

/// Truncates `s` to at most `max` characters (by Unicode scalar values).
///
/// Does not allocate if `s` is already short enough.
///
/// # Example
/// ```
/// use termz::utils::truncate;
/// assert_eq!(truncate("hello world", 5), "hello");
/// ```
pub fn truncate(s: &str, max: usize) -> &str {
    match s.char_indices().nth(max) {
        Some((idx, _)) => &s[..idx],
        None => s,
    }
}

/// Pads `s` on the right with spaces to reach `width`.
///
/// Returns the original string unchanged if it is already at least `width` characters wide.
///
/// # Example
/// ```
/// use termz::utils::pad_right;
/// assert_eq!(pad_right("hi", 5), "hi   ");
/// ```
pub fn pad_right(s: &str, width: usize) -> String {
    let len = s.chars().count();
    if len >= width {
        return s.to_string();
    }
    format!("{}{}", s, " ".repeat(width - len))
}

/// Pads `s` on the left with spaces to reach `width`.
///
/// # Example
/// ```
/// use termz::utils::pad_left;
/// assert_eq!(pad_left("hi", 5), "   hi");
/// ```
pub fn pad_left(s: &str, width: usize) -> String {
    let len = s.chars().count();
    if len >= width {
        return s.to_string();
    }
    format!("{}{}", " ".repeat(width - len), s)
}

/// Centers `s` within `width` by padding with spaces on both sides.
///
/// If the padding is odd, the extra space goes on the right.
pub fn center(s: &str, width: usize) -> String {
    let len = s.chars().count();
    if len >= width {
        return s.to_string();
    }
    let total_pad = width - len;
    let left_pad = total_pad / 2;
    let right_pad = total_pad - left_pad;
    format!("{}{}{}", " ".repeat(left_pad), s, " ".repeat(right_pad))
}

/// Wraps `s` at `width` characters, breaking on whitespace where possible.
///
/// Returns a `Vec` of lines, each at most `width` characters wide.
pub fn word_wrap(s: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();

    for word in s.split_whitespace() {
        let word_len = word.chars().count();
        let current_len = current.chars().count();

        if current_len == 0 {
            if word_len > width {
                lines.push(word.to_string());
            } else {
                current.push_str(word);
            }
        } else if current_len + 1 + word_len <= width {
            current.push(' ');
            current.push_str(word);
        } else {
            lines.push(current.clone());
            current.clear();
            if word_len > width {
                lines.push(word.to_string());
            } else {
                current.push_str(word);
            }
        }
    }

    if !current.is_empty() {
        lines.push(current);
    }

    lines
}
