use crate::types::Args;

pub fn echo(args: Args) {
    let args = args.args[1..].join(" ");
    println!("{args}");
}
