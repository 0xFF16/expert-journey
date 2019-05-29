use std::io;
use std::io::prelude::*;

fn main() {
    let mut a1 = String::new();
    let mut b2 = String::new();
    let mut c3 = String::new();

    let a: i32;
    let b: i32;
    let c: i32;

    print!("Informe o primeiro numero: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut a1).unwrap();
    a = a1.trim().parse::<i32>().unwrap();

    print!("Informe o segundo numero: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut b2).unwrap();
    b = b2.trim().parse::<i32>().unwrap();

    print!("Informe o terceiro numero: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut c3).unwrap();
    c = c3.trim().parse::<i32>().unwrap();

    println!("Quadrados da soma: {}", (a + b + c).pow(2));
}
