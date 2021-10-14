fn main() {
    let mut matrix: Vec<Vec<u8>> = Vec::with_capacity(5);
    for _ in 0..5 {
        let mut line: String = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let line: Vec<u8> = line.split_whitespace().map(|x| x.parse::<u8>().unwrap()).collect();
        matrix.push(line);
    }
    let mut coords: [i8; 2] = [0, 0];
    for y in 0..5 {
        for x in 0..5 {
            if matrix[y][x] == 1 {
                coords[0] = x as i8;
                coords[1] = y as i8;
                break;
            }
        }
    }
    print!("{}", (coords[0]-2).abs() + (coords[1]-2).abs());
}