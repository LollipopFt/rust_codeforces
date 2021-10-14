#![warn(clippy::pedantic, clippy::nursery, clippy::restriction)]

fn main() {
    use std::io::{self, Write};
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).unwrap_or_default();
    let input: Vec<char> = input.trim().to_lowercase().chars().collect();
    let mut output: String = String::new();
    for i in input {
        match i {
            'a' | 'e' | 'i' | 'o' | 'u' | 'y' => None,
            _ => {
                output += &".".to_owned();
                output += &i.to_string();
                Some(())
            }
        };
    }
    writeln!(std::io::BufWriter::new(io::stdout().lock()), "{}", output).ok();
}
