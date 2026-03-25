# termz

Terminal utilities for Rust — colors, prompts, file I/O, and string helpers.

## Modules

| Module | Description |
|--------|-------------|
| `tui`  | ANSI colors and text styles |
| `cli`  | Interactive prompts (text, confirm, select) |
| `io`   | File and stdin reading utilities |
| `utils`| String manipulation (truncate, pad, wrap) |

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
termz = "0.1"
```

### Colors & Styles

```rust
use termz::tui::{colorize, stylize, paint, Color, Style};

println!("{}", colorize("Success", Color::Green));
println!("{}", stylize("Warning", &[Style::Bold, Style::Underline]));
println!("{}", paint("Error", Color::Red, &[Style::Bold]));
```

### CLI Prompts

```rust
use termz::cli::{prompt, confirm, select};

let name = prompt("Your name: ");
if confirm("Continue? [y/n] ") {
    let idx = select("Pick a color:", &["Red", "Green", "Blue"]);
}
```

### File I/O

```rust
use termz::io::{read_lines, read_to_string};

let lines = read_lines("data.txt")?;
let content = read_to_string("config.toml")?;
```

### String Utilities

```rust
use termz::utils::{truncate, pad_right, pad_left, center, word_wrap};

let short = truncate("hello world", 5);       // "hello"
let row   = pad_right("name", 20);            // "name                "
let lines = word_wrap("a long sentence", 10);
```

## License

MIT
