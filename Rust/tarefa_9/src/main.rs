use std::io;
use std::io::prelude::*;

fn main() {
    let mut sa = String::new(); // salario atual
    let mut pr = String::new(); // porcentagem do reajuste

    let s: f32;
    let p: f32;

    print!("Informe o salario atual: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut sa).unwrap();
    s = sa.trim().parse::<f32>().unwrap();

    print!("Informe o reajuste: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut pr).unwrap();
    p = pr.trim().parse::<f32>().unwrap();

    println!("Salario apos o reajuste: {:.2}", s - p);
}
