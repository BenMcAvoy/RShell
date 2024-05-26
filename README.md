# RShell

A new cross-platform shell implementation in Rust.

## Building
```bash
git clone https://github.com/BenMcAvoy/RShell.git && cd RShell
cargo build --release
```

## Built-ins
| Command   | Purpose                     | Example                   |
|-----------|-----------------------------|---------------------------|
| `type`    | Find what type a command is | `type echo`               |
| `cd`      | Change directory            | `cd RShell`               |
| `echo`    | Print something             | `echo Path is: " $PATH "` |
| `exit`    | Quit (with exit code)       | `exit 0`                  |
| `history` | List historical commands    | `history`                 |

## Credits

This code started as a solution to the ["Build Your Own Shell" Challenge](https://app.codecrafters.io/courses/shell/overview). Shout out to CodeCrafters!
