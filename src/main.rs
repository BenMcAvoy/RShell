#![allow(unused_imports)]


use std::collections::HashMap;
use std::io::{self, Write};

#[macro_use]
mod macros;

mod builtins;
use builtins::prelude::*;

fn main() {
    let builtins = builtins_map! {
        "exit" => exit,
        "echo" => echo,
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
