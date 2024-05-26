use crate::types::Args;
use crate::path::is_in_path;

pub fn builtins(args: Args) {
    if args.builtins.contains(&args.args[1].to_string()) {
        println!("{} is a shell builtin", args.args[1]);
    } else {
        if let Some(path) = is_in_path(&args.args[1], args.path) {
            println!("{} is {}", args.args[1], path);
        } else {
            println!("{} not found", args.args[1]);
        }
    }
}
