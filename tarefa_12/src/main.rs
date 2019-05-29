use std::io;
use std::io::prelude::*;

fn main() {
    let mut a1 = String::new();
    let mut b2 = String::new();

    let a: u32;
    let b: u32;

    print!("Informe o primeiro valor: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut a1).unwrap();
    a = a1.trim().parse::<u32>().unwrap();

    print!("Informe o primeiro valor: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut b2).unwrap();
    b = b2.trim().parse::<u32>().unwrap();

    println!("Resultado: {}", a.pow(b));
}
