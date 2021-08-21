fn main() {
    let mut num: String = String::new();
    std::io::stdin().read_line(&mut num).expect("Error.");
    let num: u8 = num.trim().parse().unwrap();
    for _ in 0..num {
        let mut line: String = String::new();
        std::io::stdin().read_line(&mut line).expect("Error.");
        let line: String = line.trim().to_string();
        let first = line.chars().next().unwrap();
        if line.len()>10 {
            let last = line.chars().nth(line.len()-1).unwrap();
            println!("{}{}{}", first.to_string(),line.len()-2,last.to_string());
        } else {
            println!("{}", line);
        }
    }
}