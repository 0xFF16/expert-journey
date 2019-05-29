use std::io;
use std::io::prelude::*;

fn main() {
    let mut a1 = String::new();
    let mut b2 = String::new();

    let a: f64;
    let b: f64;

    print!("Informe o primero valor: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut a1).unwrap();
    a = a1.trim().parse::<f64>().unwrap();

    print!("Informe o segundo valor: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut b2).unwrap();
    b = b2.trim().parse::<f64>().unwrap();

    println!("{} + {}: {}", a, b, a + b);
    println!("{} - {}: {}", a, b, a - b);
    println!("{} * {}: {}", a, b, a * b);
    println!("{} / {}: {}", a, b, a / b);
}
