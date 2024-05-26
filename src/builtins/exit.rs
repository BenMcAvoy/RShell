use crate::types::Args;

pub fn exit(args: Args) {
    match args.args.len() {
        1 => std::process::exit(0),
        _ => std::process::exit(args.args[1].parse::<i32>().unwrap()),
    };
}
