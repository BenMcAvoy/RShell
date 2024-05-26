use std::io::{self, Write};
use std::path::Path;

use homedir::get_my_home;

#[macro_use]
mod macros;

mod builtins;
mod types;
mod utils;

use builtins::prelude::*;
use types::Args;

fn main() {
    let builtins = builtins_map! {
        "history" => history,
        "type" => btype,
        "exit" => exit,
        "echo" => echo,
        "cd" => cd,
    };

    let builtin_names = builtins
        .keys()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let path = std::env::var("PATH").unwrap_or_default();
    let histfile = get_my_home()
        .expect("Couldn't get home dir")
        .expect("Couldn't get home dir")
        .join(".rshell_history");

    let mut history_file = std::fs::OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(histfile)
        .unwrap();

    loop {
        printnnl!(
            "<green>{} $</green> ",
            std::env::current_dir().unwrap().display()
        );

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let args = input.trim().split_whitespace().collect::<Vec<&str>>();

        if args.len() == 0 {
            continue;
        }

        let command = args[0];

        history_file.write_all(input.as_bytes()).unwrap();

        match builtins.get(command) {
            Some(command) => command(Args {
                list: args,
                builtins: builtin_names.clone(),
                path: path.clone(),
            }),

            None => {
                if let Some(path) = utils::find_program(command, path.clone()) {
                    utils::execute_command(&args, path);
                } else {
                    if Path::new(command).exists() {
                        utils::execute_command(&args, command.to_string());
                    } else {
                        eprintln!("{}: command not found", command);
                    }
                }
            }
        }
    }
}
