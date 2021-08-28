use std::{
    time::Instant,
    io::{self, Write}
};

fn main() {
    let mut print = std::time::Duration::new(0, 0);
    for _ in 0..100 {
        let start = Instant::now();
        print!("hello");
        print+=start.elapsed()
    }
    let mut println = std::time::Duration::new(0, 0);
    for _ in 0..100 {
        let start = Instant::now();
        println!("hello");
        println+=start.elapsed()
    }
    let mut write = std::time::Duration::new(0, 0);
    for _ in 0..100 {
        let start = Instant::now();
        write!(io::BufWriter::new(io::stdout().lock()), "hello").expect("Error.");
        write+=start.elapsed()
    }
    let mut writeln = std::time::Duration::new(0, 0);
    for _ in 0..100 {
        let start = Instant::now();
        writeln!(io::BufWriter::new(io::stdout().lock()), "hello").expect("Error.");
        writeln+=start.elapsed()
    }
    println!("print!: {:?}\nprintln!: {:?}\nwrite!: {:?}\nwriteln!: {:?}", print, println, write, writeln);
}