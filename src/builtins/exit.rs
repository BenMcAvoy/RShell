use crate::types::Args;

pub fn exit(args: Args) {
    match args.0.len() {
        1 => std::process::exit(0),
        _ => std::process::exit(args.0[1].parse::<i32>().unwrap()),
    };
}
