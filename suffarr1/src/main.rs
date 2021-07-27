// suffix arrays implementation

use std::{collections::HashMap,
          convert::TryInto,
          io::stdin};

fn main() {
    let mut string: String = String::new();
    stdin().read_line(&mut string).expect("Error.");
    println!("{}", string);
    string += "$";
    println!("{}", string);
    let n: u8 = string.len().try_into().unwrap();
    let p: Vec<u8> = Vec::new();
    let c: Vec<u8> = Vec::new();
    let a: HashMap<char, u8> = HashMap::new();

    let i: u8 = 0;
    for _ in 0..n-1 {
        a.insert(string[i], i);
    }
}