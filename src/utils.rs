use std::process::Command;

/// Returns the path of the executable if it is in PATH.
/// Returns None if the executable is not in PATH.
pub fn find_program(command: &str, path: String) -> Option<String> {
    let path_delimiter = if cfg!(windows) { ";" } else { ":" };
    let dir_delimiter = if cfg!(windows) { "\\" } else { "/" };
    let path_var = path.split(path_delimiter).collect::<Vec<&str>>();

    for p in path_var {
        let p = format!("{}{}{}", p, dir_delimiter, command);
        if std::path::Path::new(&p).exists() {
            return Some(p);
        }
    }

    None
}

pub fn execute_command(args: &Vec<&str>, path: String) -> i32 {
    let mut program = Command::new(path);
    program.args(args);

    let status = program.status().unwrap();

    status.code().unwrap()
}
