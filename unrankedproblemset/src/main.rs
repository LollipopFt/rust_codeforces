#![warn(clippy::pedantic, clippy::cargo, clippy::nursery, clippy::restriction)]

fn main() {
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input: Vec<char> = input.trim().to_lowercase().chars().collect();
    let mut output: String = String::new();
    for i in input.iter() {
        match i {
            'a'|'e'|'i'|'o'|'u'|'y' => None,
            _ => {
                output+=&".".to_string();
                output+=&i.to_string();
                Some(())
            }
        };
    };
    use std::io::{self, Write};
    writeln!(io::BufWriter::new(io::stdout().lock()), "{}", output).ok();
}
