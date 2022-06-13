use std::fs::File as F;
use std::process::exit;
use colored::*;
use std::io::Read;

pub fn err(msg: String) -> String {
    format!("{} → {}", "Error".red(), msg.yellow())
}

pub fn file_err(path: String, msg: String) -> String {
    format!("{} → {}", format!("File Error (path: {})", path).red(), msg.yellow())
}

pub fn msg(text: String) -> String {
    format!("{} → {}", "Message".blue(), text.green())
}

pub struct File {
    pub name: String,
    pub content: String,
}

impl File {
    pub fn new(name: &str, content: String) -> File {
        File {
            name: name.to_string(),
            content: content,
        }
    }

    pub fn read(&mut self) -> bool {
        let file = F::open(&self.name);
        let mut contents = String::new();
        if let Ok(mut f) = file {
            match f.read_to_string(&mut contents) {
                Ok(_) => { self.content = contents; },
                Err(e) => { println!("{}", file_err(self.name.clone(), e.to_string())) },
            };
            return true;
        } else if let Err(e) = file {
            println!("{}", file_err(self.name.clone(), e.to_string()));
        }
        return false;
    }
}

pub struct Grep {
    pub file: File,
    pub pattern: String,
}

impl Grep {
    pub fn new(file: File, pattern: String) -> Grep {
        Grep {
            file: file,
            pattern: pattern,
        }
    }

    pub fn run(&mut self) {
        println!("{}", msg("Command: Grep".to_string()));
        if !self.file.read() { exit(1); }
        let mut lines = self.file.content.lines();
        let mut count = 0;
        while let Some(line) = lines.next() {
            if line.contains(&self.pattern) {
                println!("{}: {}", "Match".blue(), line.purple());
                count += 1;
            }
        };
        if count == 0 {
            println!("{}", "No matches found");
        } else {
            println!("{}: {}", "Total match count".blue(), count);
        }
    }
}

pub struct Cat {
    pub file: File,
}

impl Cat {
    pub fn new(file: File) -> Cat {
        Cat {
            file: file,
        }
    }

    pub fn run(&mut self) {
        println!("{}", msg("Command: Cat".to_string()));
        if !self.file.read() { exit(1); }
        println!("{}", self.file.content.purple());
    }
}

pub struct Diff {
    pub file1: File,
    pub file2: File,
}

impl Diff {
    pub fn new(file1: File, file2: File) -> Diff {
        Diff {
            file1: file1,
            file2: file2,
        }
    }

    pub fn run(&mut self) {
        println!("{}", msg("Command: Diff".to_string()));
        if !self.file1.read() { exit(1); };
        if !self.file2.read() { exit(1); };
        println!("{}", msg(format!("Comparing files: {}, {}", self.file1.name, self.file2.name)));
        let mut lines1 = self.file1.content.lines().enumerate();
        let mut lines2 = self.file2.content.lines().enumerate();
        let mut line1 = lines1.next();
        let mut line2 = lines2.next();
        let mut diff = 0;
        while line1.is_some() && line2.is_some() {
            if line1.unwrap().1 != line2.unwrap().1 {
                if line1.unwrap().0 == line2.unwrap().0 {
                    let n = line1.unwrap().0 + 1;
                    println!("{} {}: {} -- {}", "Line".blue(), n, line1.unwrap().1.purple(), line2.unwrap().1.purple());
                    diff += 1;
                }
            }
            line1 = lines1.next();
            line2 = lines2.next();
        }
        if diff == 0 {
            println!("{}", "Files are identical".blue());
        }
    }
}

pub struct Help { }

impl Help {
    pub fn new() -> Help {
        Help { }
    }

    pub fn run(&mut self) {
        println!("{}: {}", "Usage".blue(), "ftools <COMMAND> <ARGS>...".green());
        println!("{}:", "Commands".blue());
        println!("  g, grep {}  {}    Find lines matching PATTERN", "<FILE>".magenta(), "<PATTERN>".magenta());
        println!("  c, cat  {}               Print the contents of FILE", "<FILE>".magenta());
        println!("  d, diff {} {}      Compare FILE1 and FILE2", "<FILE1>".magenta(), "<FILE2>".magenta());
        println!("  h, help                      Print this help message");
    }
}

pub enum CommandType {
    Grep,
    Cat,
    Diff,
    Help,
}

pub enum Command {
    Grep(Grep),
    Cat(Cat),
    Diff(Diff),
    Help(Help),
}

pub struct ArgParser {
    pub arguments: Vec<String>,
}

impl ArgParser {
    pub fn new(args: Vec<String>) -> ArgParser {
        ArgParser {
            arguments: args,
        }
    }

    pub fn parse(&self) -> Result<Command, String> {
        let mut args = self.arguments.clone();
        let cmd = args.remove(0);
        let command: Result<CommandType, String> = match cmd.as_str() {
            "grep" | "g" => Ok(CommandType::Grep),
            "cat" | "c" => Ok(CommandType::Cat),
            "diff" | "d" => Ok(CommandType::Diff),
            "help" | "h" => Ok(CommandType::Help),
            _ => Err(err("Invalid command".to_string())),
        };
        match command {
            Ok(c) => match c {
                CommandType::Grep => {
                    if args.len() < 2 {
                        return Err(err("Not enough arguments to grep".to_string()));
                    }
                    let file = File::new(&args[0], String::new());
                    let pattern = args[1].clone();
                    Ok(Command::Grep(Grep::new(file, pattern)))
                },
                CommandType::Cat => {
                    if args.len() < 1 {
                        return Err(err("Not enough arguments to cat".to_string()));
                    }
                    let file = File::new(&args[0], String::new());
                    Ok(Command::Cat(Cat::new(file)))
                },
                CommandType::Diff => {
                    if args.len() < 2 {
                        return Err(err("Not enough arguments to diff".to_string()));
                    }
                    let file1 = File::new(&args[0], String::new());
                    let file2 = File::new(&args[1], String::new());
                    Ok(Command::Diff(Diff::new(file1, file2)))
                },
                CommandType::Help => {
                    Ok(Command::Help(Help::new()))
                },
            },
            Err(e) => Err(e),
        }
    }
}
