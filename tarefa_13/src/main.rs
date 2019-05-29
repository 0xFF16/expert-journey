use std::io;
use std::io::prelude::*;

fn main() {
    let mut a1 = String::new();
    let a: i32;

    print!("Informe um valor: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut a1).unwrap();
    a = a1.trim().parse::<i32>().unwrap();

    println!("sucessor: {}\nantecessor: {}", a + 1, a - 1);
}
