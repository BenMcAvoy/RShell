use std::env;
use std::path::PathBuf;
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

/// Executes a command with the given arguments. Returns the exit code of the command.
pub fn execute_command(args: &[&str], path: String) -> i32 {
    let mut program = Command::new(path);
    program.args(args);

    let status = program.status().unwrap();

    status.code().unwrap()
}

/// Returns the path to the user's home directory.
pub fn get_home() -> Option<PathBuf> {
    // Attempt to read the HOME environment variable
    if let Ok(home_env) = env::var("HOME") {
        return Some(PathBuf::from(&home_env));
    }

    // Fallback to USERPROFILE on Windows
    if cfg!(target_os = "windows") {
        if let Ok(user_profile) = env::var("USERPROFILE") {
            return Some(PathBuf::from(&user_profile));
        }
    }

    // Fallback to common Unix paths
    if cfg!(not(target_os = "windows")) {
        let username = env::var("USERNAME").unwrap_or_else(|_| String::from("default"));
        let mut path = PathBuf::from("/home");
        path.push(&username);
        return Some(path);
    }

    None
}
