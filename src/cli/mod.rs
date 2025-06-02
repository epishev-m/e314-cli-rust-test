mod commands;
mod repositories;
mod project;
mod unity_project;
mod packages_manifest;
mod upm;
mod dot_net_project;

use clap::Command;

pub fn run() {
    let command = build();
    handle(command);
}

fn build() -> Command {
    Command::new("e314-cli")
        .version("1.0.0")
        .author("Maksim Epishev - epishev.m@mail.ru")
        .about("Tool for working with E314")
        .subcommand(commands::build_list())
        .subcommand(commands::build_clone())
        .subcommand(commands::build_new())
}

fn handle(command: Command) {
    match command.get_matches().subcommand() {
        Some(("list", _)) => commands::execute_list(),
        Some(("clone", args)) => commands::execute_clone(args),
        Some(("new", _)) => commands::execute_new(),
        _ => {}
    }
}