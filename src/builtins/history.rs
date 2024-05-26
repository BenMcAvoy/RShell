use crate::types::Args;
use std::io::Read;

pub fn history(args: Args) {
    if args.args.len() > 2 {
        eprintln!("history: too many arguments");
        return;
    } else if args.args.len() == 2 {
        if args.args[1] == "clear" {
            let histfile = std::env::var("HOME").unwrap_or_default().to_string() + "/.rshell_history";

            std::fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(histfile)
                .unwrap();

            return;
        } else {
            eprintln!("history: invalid argument");
            return;
        }
    }

    let histfile = std::env::var("HOME").unwrap_or_default().to_string() + "/.rshell_history";
    let mut history_file = std::fs::OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(histfile)
        .unwrap();

    let mut contents = String::new();
    history_file.read_to_string(&mut contents).unwrap();

    print!("{}", contents);
}
