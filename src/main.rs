mod config;
use std::collections::HashSet;
use std::env;
use std::io;
use std::process;

use clap::Command;
use clap::CommandFactory;
use clap::{Args, Parser, Subcommand};
use clap_complete::generate;
use clap_complete::Generator;
use clap_complete::Shell;
use config::config::{RutConfig, Window};
use tmux_interface::Sessions;
use tmux_interface::SESSION_ALL;
use tmux_interface::{Error, TmuxCommand, TmuxOutput};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[arg(short, long)]
    /// Sets a custom config file
    file: Option<String>,
    #[command(subcommand)]
    command: Option<Commands>,
    // Generates shell completion
    #[arg(long = "generate", value_enum)]
    generator: Option<Shell>,
}

#[derive(Subcommand, PartialEq)]
enum Commands {
    /// Attaches to a given session
    Attach(SessionCommand),
    /// Lists sessions in config
    List(List),
    /// Starts a given session, detached
    Start(SessionCommand),
}

#[derive(Args, PartialEq)]
struct SessionCommand {
    name: Option<String>,
}

#[derive(Args, PartialEq)]
struct List {
    #[arg(long)]
    /// Get a list of all running sessions from your config
    running: bool,
}

fn send_command(window: &Window, tmux: TmuxCommand) -> Result<TmuxOutput, Error> {
    let command = match &window.command {
        Some(c) => format!("cd {} && {}", window.dir, c),
        None => format!("cd {}", window.dir),
    };

    tmux.send_keys().key(&command).key("C-m").output()
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

fn main() {
    let cli = Cli::parse();
    if cli.generator == None && cli.file == None && cli.command == None {
        let mut cmd = Cli::command();
        cmd.print_help().unwrap();
        return;
    }
    if let Some(generator) = cli.generator {
        let mut cmd = Cli::command();
        print_completions(generator, &mut cmd);
        return;
    }
    let home = env::var("HOME").unwrap();
    let config_home = format!("{}/.config/ruts/ruts.yaml", home);
    let file = cli.file.as_deref().unwrap_or(&config_home);
    let data = match std::fs::read_to_string(file) {
        Ok(data) => data,
        Err(_) => {
            println!("Error reading config");
            return;
        }
    };
    let decoded: RutConfig = serde_yaml::from_str(&data).unwrap();
    let workspaces = decoded.workspaces.as_ref().unwrap();
    match &cli.command.unwrap() {
        Commands::List(list) => {
            // Create hash sets of the workspaces and the sessions
            let workspace_names: HashSet<String> =
                workspaces.iter().map(|w| w.name.clone()).collect();
            let sessions: HashSet<String> = Sessions::get(SESSION_ALL)
                .unwrap()
                .into_iter()
                .map(|session| session.name.unwrap().clone())
                .collect();
            let intersect = workspace_names.intersection(&sessions);
            // Find the intersection and print them out if running
            if list.running {
                for workspace in intersect {
                    println!("{}", workspace);
                }
            } else {
                let vec_test: Vec<&String> = intersect.collect();
                for workspace in workspaces {
                    if vec_test.contains(&&workspace.name) {
                        println!("*{}", workspace.name);
                    } else {
                        println!("{}", workspace.name);
                    }
                }
            }
            process::exit(0);
        }
        Commands::Attach(attach) => {
            let item = attach.name.as_ref().unwrap();
            let tmux = TmuxCommand::new();
            let sessions = Sessions::get(SESSION_ALL).unwrap();
            match sessions
                .into_iter()
                .find(|i| i.name.as_ref().unwrap() == item)
            {
                Some(session) => {
                    let name = session.name.unwrap();
                    tmux.attach_session().target_session(name).output().unwrap();
                }
                None => {
                    println!("Workspace \"{}\", not found.", item);
                    process::exit(1);
                }
            }
        }
        Commands::Start(start) => {
            let item = start.name.as_ref().unwrap();
            match workspaces.iter().find(|i| i.name == *item) {
                Some(workspace) => {
                    let tmux = TmuxCommand::new();
                    let mut windows = workspace.windows.clone().unwrap();

                    let workspace_name = &workspace.name;
                    let first_window = &windows[0];
                    tmux.new_session()
                        .session_name(workspace_name)
                        .window_name(&first_window.name)
                        .detached()
                        .output()
                        .unwrap();
                    send_command(first_window, tmux.clone()).unwrap();

                    windows.remove(0);

                    // new_session.session_name(workspace_name);
                    for window in windows {
                        tmux.new_window()
                            .window_name(&window.name)
                            .output()
                            .unwrap();
                        send_command(&window, tmux.clone()).unwrap();
                    }
                    process::exit(0);
                }
                None => {
                    println!("Config item not found!");
                    process::exit(1);
                }
            }
        }
    }
}
