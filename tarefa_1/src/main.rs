use std::io;
use std::io::prelude::*;

fn main() {
    let mut fah = String::new();
    let f: i64;
    print!("Entre com o valor em fahrenheit: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut fah).unwrap();
    f = fah.trim().parse::<i64>().unwrap();

    println!("{}° C", ((f - 32) * 5) / 9);
}
