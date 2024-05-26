use crate::types::Args;
pub fn builtins(args: Args) {
    if args.1.contains(&args.0[1].to_string()) {
        println!("{} is a shell builtin", args.0[1]);
    } else {
        println!("{} not found", args.0[1]);
    }
}
