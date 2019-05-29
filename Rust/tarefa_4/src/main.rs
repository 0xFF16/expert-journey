use std::io;
use std::io::prelude::*;

fn main() {
    let mut comp = String::new();
    let mut lar = String::new();
    let mut alt = String::new();

    let c: f32;
    let l: f32;
    let a: f32;
    let volume: f32;

    print!("Informe o comprimeito do retangulo: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut comp).unwrap();
    c = comp.trim().parse::<f32>().unwrap();

    print!("Informe a altura do retangulo: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut lar).unwrap();
    l = lar.trim().parse::<f32>().unwrap();

    print!("Informe a largura do retangulo: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut alt).unwrap();
    a = alt.trim().parse::<f32>().unwrap();

    volume = c * l * a;

    println!("Volume do retangulo: {}", volume);
}
