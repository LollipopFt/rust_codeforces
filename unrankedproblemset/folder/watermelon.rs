fn main() {
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).expect("Error.");
    let num: u8 = input.trim().parse().unwrap();
    if num%2==0 && num>2 {
        println!("YES");
    } else {
        println!("NO");
    };
}
