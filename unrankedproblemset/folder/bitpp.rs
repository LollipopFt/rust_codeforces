fn main() {
    let mut instructn: String = String::new();
    std::io::stdin().read_line(&mut instructn).unwrap();
    let instructn: u8 = instructn.trim().parse().unwrap();
    let mut x: i16 = 0;
    for _ in 0..instructn {
        let mut input: String = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().as_bytes();
        if input[1] == b'+' {
            x += 1
        } else {
            x -= 1
        }
    }
    use std::io::{self, Write};
    write!(io::BufWriter::new(io::stdout()), "{}", x).expect("Error.");
}