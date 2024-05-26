#![allow(unused_imports)]

use std::collections::HashMap;
use std::io::{self, Write};

macro_rules! printnnl {($($arg:tt)*) => {print!($($arg)*);io::stdout().flush().unwrap();}}

fn main() {
    let commands: HashMap<&str, Box<dyn Fn(Vec<&str>) -> ()>> = HashMap::new();

    loop {
        printnnl!("$ ");

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        match commands.get(input.trim()) {
            Some(command) => command(vec![]),
            None => println!("{}: command not found", input.trim()),
        }
    }
}
