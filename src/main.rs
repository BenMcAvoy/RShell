use std::io::{self, Write};

#[macro_use]
mod macros;

mod path;
mod types;
mod builtins;

use builtins::prelude::*;
use types::Args;

fn main() {
    let builtins = builtins_map! {
        "exit" => exit,
        "echo" => echo,
        "type" => builtins,
    };

    let builtin_names = builtins
        .keys()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let path = std::env::var("PATH").unwrap_or_default();

    loop {
        printnnl!("$ ");

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let args = input.trim().split_whitespace().collect::<Vec<&str>>();
        let command = args[0];

        match builtins.get(command) {
            Some(command) => command(Args {
                args,
                builtins: builtin_names.clone(),
                path: path.clone(),
            }),

            None => println!("{}: command not found", input.trim()),
        }
    }
}
