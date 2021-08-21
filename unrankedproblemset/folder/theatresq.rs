fn main() {
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).expect("Error.");
    let input: Vec<&str> = input.trim().split_whitespace().collect();
    let mut numvec: Vec<u64> = vec![];
    for i in input.iter() {
        numvec.push(i.parse().unwrap())
    }
    let breadth = numvec[0]/numvec[numvec.len()-1] + modcheck(&numvec, 0);
    let length = numvec[1]/numvec[numvec.len()-1] + modcheck(&numvec, 1);
    print!("{}", breadth*length);
}

fn modcheck(numvec: &[u64], pos: usize) -> u64 {
    if numvec[pos]%numvec[numvec.len()-1]!=0 {
        1
    } else {
        0
    }
}