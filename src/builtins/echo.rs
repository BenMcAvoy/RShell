use crate::types::Args;

pub fn echo(args: Args) {
    let mut list: Vec<String> = args.list.iter().map(|v| v.to_string()).collect();

    for item in list.iter_mut() {
        if item.starts_with('$') {
            let var_name = item[1..].to_string();
            let var_value = match std::env::var(&var_name) {
                Ok(v) => v,
                Err(_) => {
                    eprintln!("echo: ${}: not set", var_name);
                    return;
                }
            };

            *item = var_value;
        }
    }

    let args = list[1..].join(" ");
    println!("{}", args);
}
