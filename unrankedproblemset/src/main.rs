fn main() {
    let mut matrix: Vec<Vec<u8>> = Vec::new();
    for _ in 0..5 {
        let mut line: String = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let line: Vec<u8> = line.split_whitespace().map(|x| x.parse::<u8>().unwrap()).collect();
        matrix.push(line);
    }
    print!("{:?}", matrix);
}