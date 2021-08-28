fn main() {
    let mut board: String = String::new();
    std::io::stdin().read_line(&mut board).unwrap();
    let board: Vec<u16> = board.split_whitespace().map(|x| x.parse().unwrap()).collect();
    print!("{}", board[0]*board[1]/2);
}