#[macro_use]
extern crate serde_derive;

use std::env;
use std::fs::File;
use std::io;
use std::io::Read;
use std::process;

#[derive(Serialize, Deserialize)]
struct Pessoa {
    nome: Vec<String>,
    idade: Vec<u8>,
}

fn main() -> io::Result<()> {
    let argv: Vec<String> = env::args().collect();
    match argv.len() {
        2 => {
            let mut arquivo = File::open(argv[1].clone())?;
            let mut buffer = String::new();
            arquivo.read_to_string(&mut buffer)?;
            let p: Pessoa = serde_json::from_str(&buffer)?;
            p.nome
                .iter()
                .map(|x| x)
                .for_each(|x| println!("Nome: {}", x));
            p.idade
                .iter()
                .map(|x| x)
                .for_each(|x| println!("Idade: {}", x));
        }
        _ => {
            println!(
                "Modo de Uso:\n\t $ {} <arquivo>\nEx:\t $ {} json.json",
                argv[0], argv[0]
            );
            process::exit(1);
        }
    };
    Ok(())
}
