# RUTS
A Rust-based tmux sessionizer

## Usage 
ruts [OPTIONS] [COMMAND]

Commands:
- attach  Attaches to a given session
- list    Lists sessions in config
- start   Starts a given session, detached
- help    Print this message or the help of the given subcommand(s)

Options:
- -f, --file <FILE>           Sets a custom config file
- --generate <GENERATOR>      [possible values: bash, elvish, fish, powershell, zsh]
- -h, --help                  Print help information
- -V, --version               Print version information

