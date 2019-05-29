use std::io;
use std::io::prelude::*;

fn main() {
    let mut var = String::new();
    let v: u64;
    print!("Informe um numero: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut var).unwrap();
    v = var.trim().parse::<u64>().unwrap();
    println!("{} ^ 2: {}", v, v.pow(2) as u32);
}
