use crate::types::Args;
use crate::path::is_in_path;

pub fn btype(args: Args) {
    if args.list.len() > 1 {
        let command = &args.list[1];

        if args.builtins.contains(&command.to_string()) {
            println!("{} is a shell builtin", command);
        } else if let Some(full_path) = is_in_path(command.to_string(), args.path) {
            println!("{} is {}", command, full_path);
        } else {
            println!("{} not found", command);
        }
    } else {
        eprintln!("type: missing argument");
    }
}
