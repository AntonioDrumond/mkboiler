mod nix;

use std::env;
use crate::nix::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut i = 1;
    while i < args.len() {
        // println!("{}", args[i]);
        match args[i].as_str() {
            "default.nix" => default_nix(),
            _ => eprintln!("Unknown boilerplate: {}", args[i]),
        };
        i += 1;
    }
}
