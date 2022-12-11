#[derive(Debug)]
pub enum CommandLine {
    Cd { argument: String },
    Ls
}

#[derive(Debug)]
pub enum OutputLine {
    File { name: String, size: usize },
    Directory { name: String }
}

impl OutputLine {
    pub fn get_name(&self) -> &str {
        match self {
            OutputLine::File { name, size: _ } => name,
            OutputLine::Directory { name } => name
        }
    }
}

#[derive(Debug)]
pub enum TerminalLine {
    Command(CommandLine),
    Output(OutputLine)
}

#[derive(Debug)]
pub enum Command {
    Cd { argument: String },
    Ls { output: Vec<OutputLine> }
}

impl TerminalLine {
    pub fn from_line(line: &str) -> TerminalLine {
        let split = line.split(" ").collect::<Vec<&str>>();

        if line.chars().nth(0) == Some('$') {
            // This line is a command
            let cmd = split[1];
            let arg = match split.get(2) {
                Some(x) => Some(String::from(*x)),
                None => None
            };

            match cmd {
                "cd" => TerminalLine::Command(CommandLine::Cd { argument: arg.unwrap() }),
                "ls" => TerminalLine::Command(CommandLine::Ls),
                _ => panic!("Invalid command: {}", cmd)
            }
        } else {
            // This line is output
            if split[0] == "dir" {
                let name = split[1].to_string();
                TerminalLine::Output(OutputLine::Directory { name })
            } else {
                let size = split[0].parse::<usize>().unwrap();
                let name = split[1].to_string();
                TerminalLine::Output(OutputLine::File { size, name }) 
            }
        }
    }
}


