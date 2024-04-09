use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let quera = &args[1];
    let filename = &args[2];
    println!("{}", quera);
    println!("{}", filename);

    let context = fs::read_to_string(filename).unwrap();
    println!("\n-------------[{}] context-------------", filename);
    println!("{}", context);
}
