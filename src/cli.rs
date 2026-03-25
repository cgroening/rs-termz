//! Interactive CLI prompt helpers.

use std::io::{self, Write};

/// Prints `label` and reads a line of user input from stdin.
///
/// Leading and trailing whitespace is trimmed from the result.
///
/// # Example
/// ```no_run
/// use termz::cli::prompt;
/// let name = prompt("Enter your name: ");
/// ```
pub fn prompt(label: &str) -> String {
    print!("{label}");
    io::stdout().flush().unwrap_or(());
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap_or(0);
    buf.trim().to_string()
}

/// Displays `label` and asks the user to confirm with `y/n`.
///
/// Returns `true` if the user types `y` or `Y`, `false` otherwise.
///
/// # Example
/// ```no_run
/// use termz::cli::confirm;
/// if confirm("Delete file? [y/n] ") {
///     println!("Deleted.");
/// }
/// ```
pub fn confirm(label: &str) -> bool {
    let answer = prompt(label);
    matches!(answer.to_lowercase().as_str(), "y" | "yes")
}

/// Displays `label` with a numbered list of `options` and returns the chosen index.
///
/// Keeps prompting until a valid number is entered.
///
/// # Example
/// ```no_run
/// use termz::cli::select;
/// let idx = select("Choose a color:", &["Red", "Green", "Blue"]);
/// ```
pub fn select(label: &str, options: &[&str]) -> usize {
    println!("{label}");
    for (i, opt) in options.iter().enumerate() {
        println!("  {}. {}", i + 1, opt);
    }
    loop {
        let input = prompt("> ");
        if let Ok(n) = input.parse::<usize>() {
            if n >= 1 && n <= options.len() {
                return n - 1;
            }
        }
        println!("Please enter a number between 1 and {}.", options.len());
    }
}
