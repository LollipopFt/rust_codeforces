fn main() {
    let mut stri: String = String::new();
    std::io::stdin().read_line(&mut stri).unwrap();
    let mut stri1: String = String::new();
    std::io::stdin().read_line(&mut stri1).unwrap();
    let stri: String = stri.trim().to_lowercase();
    let stri1: String = stri1.trim().to_lowercase();
    use std::{io::{self, Write}, cmp::Ordering};
    writeln!(io::BufWriter::new(io::stdout().lock()), "{}",
         match stri.cmp(&stri1) {
         Ordering::Greater => 1,
         Ordering::Less => -1,
         Ordering::Equal => 0
    }).ok();
}
