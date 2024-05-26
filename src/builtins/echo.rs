pub fn echo(args: Vec<&str>) {
    let args = args[1..].join(" ");
    println!("{args}");
}
