use std::{env, fs};
use minigrep;
fn main() {
    let config = minigrep::Config::new(env::args().collect());
    println!("{:?}", config);

    let context = fs::read_to_string(&config.filename).unwrap();
    println!("\n-------------[{}] context-------------", config.filename);
    println!("{}", context);
}
