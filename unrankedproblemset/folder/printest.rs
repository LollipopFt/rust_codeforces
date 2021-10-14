use std::{
    time::Instant,
    io::{self, Write}
};

fn main() {
    println!("Multi Test:");
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
    let mut handle = io::BufWriter::new(io::stdout());
    for _ in 0..100 {
        let start = Instant::now();
        write!(handle, "hello").ok();
        write+=start.elapsed()
    }
    io::stdout().flush().ok();
    let mut writeln = std::time::Duration::new(0, 0);
    let mut handle = io::BufWriter::new(io::stdout());
    for _ in 0..100 {
        let start = Instant::now();
        writeln!(handle, "hello").ok();
        writeln+=start.elapsed()
    }
    io::stdout().flush().ok();
    println!("print!: {:?}\nprintln!: {:?}\nwrite!: {:?}\nwriteln!: {:?}", print, println, write, writeln);
}
