use std::io::stdin;

fn main() {
    let mut num: String = String::new();
    stdin().read_line(&mut num).expect("Error.");
    let num: u16 = num.trim().parse().unwrap();
    let mut total: u16 = 0;
    for _ in 0..num {
        let mut st: String = String::new();
        stdin().read_line(&mut st).expect("Error.");
        let st: Vec<&str> = st.trim().split_whitespace().collect();
        let mut btotal: u16 = 0;
        for no in st.iter() {
            btotal += no.parse::<u16>().unwrap()
        }
        if btotal > 1 {
            total += 1
        } else {}
    }
    print!("{}", total);
}