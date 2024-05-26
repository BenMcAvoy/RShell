pub fn exit(args: Vec<&str>) {
    match args.len() {
        1 => std::process::exit(0),
        _ => std::process::exit(args[1].parse::<i32>().unwrap()),
    };
}
