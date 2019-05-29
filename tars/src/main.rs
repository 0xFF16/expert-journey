extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::fs::File;
use std::io;
use std::io::prelude::*;

#[derive(Serialize, Deserialize)]
struct Pessoa {
    nome: String,
    idade: u8,
    lang: String
}

fn main() -> io::Result<()> {
    let mut file = File::open("info.json")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    let p: Pessoa = serde_json::from_str(&buffer)?;
    println!("Nome: {}\nIdade: {}\nLang: {}", p.nome, p.idade, p.lang);
    Ok(())
}
