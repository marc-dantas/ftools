use std::env::args;
use std::process::exit;
mod tools;

fn main() {
    let arguments: Vec<String>;
    if args().len() > 1 {
        arguments = args().skip(1).collect::<Vec<String>>();
    } else {
        println!("Error: No arguments provided");
        tools::Help::new().run();
        exit(1);
    }
    let cmd: tools::Command = match tools::ArgParser::new(arguments).parse() {
        Ok(c) => c,
        Err(e) => {
            println!("Error: {}", e);
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
