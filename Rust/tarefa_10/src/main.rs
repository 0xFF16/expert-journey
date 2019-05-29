use std::io;
use std::io::prelude::*;

fn main() {
    let mut raio = String::new();
    let r: f64;
    let vol: f64;

    print!("Informe o raio: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut raio).unwrap();
    r = raio.trim().parse::<f64>().unwrap();

    vol = (3.14159265 * r).powf(2.0);

    println!("Area de circunferencia: {:.2}", vol);
}
