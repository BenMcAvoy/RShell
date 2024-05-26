#![allow(unused_imports)]

use std::collections::HashMap;
use std::io::{self, Write};

macro_rules! printnnl {($($arg:tt)*) => {print!($($arg)*);io::stdout().flush().unwrap();}}
macro_rules! builtins_map {
    () => (std::collection::HashMap::new());
    ($($key:expr => $value:expr), + $(,)?) => ({
        let mut map = std::collections::HashMap::new();
        $(map.insert($key, $value);)+
        map
    })
}

fn exit(args: Vec<&str>) {
    match args.len() {
        1 => std::process::exit(0),
        _ => std::process::exit(args[1].parse::<i32>().unwrap()),
    }
}

fn main() {
    let builtins = builtins_map! {
        "exit" => exit,
    };

    loop {
        printnnl!("$ ");

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let args = input.trim().split_whitespace().collect::<Vec<&str>>();
        let command = args[0];

        match builtins.get(command) {
            Some(command) => command(args),
            None => println!("{}: command not found", input.trim()),
        }
    }
}
