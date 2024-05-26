use crate::types::Args;

pub fn echo(args: Args) {
    let args = args.0[1..].join(" ");
    println!("{args}");
}
