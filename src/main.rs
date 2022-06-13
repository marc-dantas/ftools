use std::env::args;
use std::process::exit;
use colored::*;
mod tools;

fn head() {
    println!("{} {}", "Welcome to".blue(), "ftools".blue().bold());
}

fn main() {
    head();
    let arguments: Vec<String>;
    if args().len() > 1 {
        arguments = args().skip(1).collect::<Vec<String>>();
    } else {
        println!("{}: {}", "Error".red(), "No arguments provided".yellow());
        tools::Help::new().run();
        exit(1);
    }
    let cmd: tools::Command = match tools::ArgParser::new(arguments).parse() {
        Ok(c) => c,
        Err(e) => {
            println!("{}", e);
            tools::Help::new().run();
            exit(1);
        }
    };
    match cmd {
        tools::Command::Grep(mut g) => g.run(),
        tools::Command::Cat(mut c) => c.run(),
        tools::Command::Diff(mut d) => d.run(),
        tools::Command::Help(mut h) => h.run(),
    };
}
