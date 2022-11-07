mod config;
use std::env;
use std::process;

use clap::{Args, Parser, Subcommand};
use config::config::{RutConfig, Window};
use tmux_interface::Sessions;
use tmux_interface::SESSION_ALL;
use tmux_interface::{Error, TmuxCommand, TmuxOutput};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    file: Option<String>,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Attach(SessionCommand),
    List(List),
    Start(SessionCommand),
}

#[derive(Args)]
struct SessionCommand {
    name: Option<String>,
}

#[derive(Args)]
struct List {}

fn send_command(window: &Window, tmux: TmuxCommand) -> Result<TmuxOutput, Error> {
    let command = match &window.command {
        Some(c) => format!("cd {} && {}", window.dir, c),
        None => format!("cd {}", window.dir),
    };

    tmux.send_keys().key(&command).key("C-m").output()
}

fn main() {
    let cli = Cli::parse();
    let home = env::var("HOME").unwrap();
    let config_home = format!("{}/.config/ruts/ruts.yaml", home);
    let file = cli.file.as_deref().unwrap_or(&config_home);
    let data = std::fs::read_to_string(file).expect("Error reading");
    let decoded: RutConfig = serde_yaml::from_str(&data).unwrap();
    let workspaces = decoded.workspaces.as_ref().unwrap();
    match &cli.command {
        Commands::List(..) => {
            for workspace in workspaces {
                println!("{}", workspace.name);
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
                    tmux.attach_session().target_session(name);
                }
                None => {
                    println!("Workspace \"{}\", not found.", item);
                    process::exit(1);
                }
            }

            match workspaces.iter().find(|i| i.name == *item) {
                Some(workspace) => {
                    let tmux = TmuxCommand::new();
                    let workspace_name = &workspace.name;
                    tmux.attach_session()
                        .target_session(workspace_name)
                        .output()
                        .unwrap();
                }
                None => {}
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
