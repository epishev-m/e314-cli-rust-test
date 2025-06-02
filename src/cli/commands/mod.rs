use clap::{Arg, ArgMatches, Command};
use colored::Colorize;

mod clone;
mod list;
mod new;

pub fn execute_list() {
    match list::execute() {
        Ok(_) => println!("\n"),
        Err(_) => println!("{}", "List error\n".red())
    }
}

pub fn build_list() -> Command {
    Command::new("list")
        .about("Display the list of e314 modules")
}

pub fn execute_clone(args: &ArgMatches) {
    let idx = args.get_one::<String>("index");
    match idx {
        Some(index) => match clone::execute(Some(index)) {
            Ok(_) => println!("\n{}", "Clone success\n".green()),
            Err(_) => println!("{}\n", "Clone error\n".red())
        },
        None => match clone::execute(None) {
            Ok(_) => println!("\n{}", "Clone success\n".green()),
            Err(_) => println!("{}", "Clone error\n".red())
        }
    }
}

pub fn build_clone() -> Command {
    Command::new("clone")
        .about("Clone a repository by index")
        .arg(
            Arg::new("index")
                .help("The index of the module (optional - clones all if not specified)")
                .index(1))
}

pub fn execute_new() {
    match new::execute() {
        Ok(_) => {},
        Err(_) => println!("{}", "Init error\n".red())
    }
}

pub fn build_new() -> Command {
    Command::new("new")
        .about("Create a new project")
}