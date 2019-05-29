use std::io;
use std::io::prelude::*;

fn main() {
    let mut cel = String::new();
    let c: i64;
    print!("Entre com o valor em celsius: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut cel).unwrap();
    c = cel.trim().parse::<i64>().unwrap();

    println!("{}° F", (9 * c + 160) / 5);
}
