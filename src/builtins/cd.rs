use crate::types::Args;
use std::env::set_current_dir;

pub fn cd(args: Args) {
    if args.args.len() == 1 {
        let home = std::env::var("HOME").unwrap();
        set_current_dir(home).unwrap();
    } else {
        let path = args.args[1];
        set_current_dir(path).unwrap();
    }
}
