use crate::types::Args;

pub fn echo(args: Args) {
    let args = args.list[1..].join(" ");
    println!("{args}");
}
