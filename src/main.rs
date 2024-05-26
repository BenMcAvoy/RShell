use std::io::{self, Write};

mod types;

#[macro_use]
mod macros;

mod builtins;
use builtins::prelude::*;

fn main() {
    let builtins = builtins_map! {
        "exit" => exit,
        "echo" => echo,
        "type" => builtins,
    };

    let builtin_names = builtins.keys().map(|s| s.to_string()).collect::<Vec<String>>();

    loop {
        printnnl!("$ ");

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let args = input.trim().split_whitespace().collect::<Vec<&str>>();
        let command = args[0];

        match builtins.get(command) {
            Some(command) => command((args, builtin_names.clone())),
            None => println!("{}: command not found", input.trim()),
        }
    }
}
