fn main() {
    let mut n_k: String = String::new();
    std::io::stdin().read_line(&mut n_k).expect("Error.");
    let n_k: Vec<&str> = n_k.trim().split_whitespace().collect();
    let mut un_k: Vec<usize> = vec![];
    for st in n_k.iter() {
        un_k.push(st.parse().unwrap())
    }
    let mut contestant: String = String::new();
    std::io::stdin().read_line(&mut contestant).expect("Error.");
    let contestant: Vec<&str> = contestant.trim().split_whitespace().collect();
    let mut ucontestant: Vec<u8> = vec![];
    for st in contestant.iter() {
        ucontestant.push(st.parse().unwrap())
    }
    ucontestant.sort_unstable();
    ucontestant.reverse();
    let mut passes: u8 = 0;
    let lowest: u8 = ucontestant[un_k[1]-1];
    for i in ucontestant.iter() {
        if i >= &lowest  && i > &0 {
            passes += 1
        }
    }
    print!("{}", passes)
}