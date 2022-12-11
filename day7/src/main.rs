use std::fs::read_to_string as read;

mod filesystem;
mod terminal;
use terminal::*;
use filesystem::*;

fn main() {
    let commands = parse(read("input.txt").unwrap());
    let fs = Filesystem::from_commands(commands);

    let mut part_1 = 0;
    for node in fs.get_nodes().iter() {
        let size = node.get_size(&fs);
        if node.is_directory() && size < 100000 {
            part_1 += size;
        }
    }

    let total_size = 70000000;
    let update_size = 30000000;
    let used = fs.get_node(fs.get_root()).get_size(&fs);
    let remaining = total_size - used;
    let space_to_free = update_size - remaining;

    let mut dir_sizes: Vec<usize> = fs.get_nodes().iter()
        .filter(|node| node.is_directory())
        .map(|node| node.get_size(&fs))
        .filter(|size| size >= &space_to_free)
        .collect();

    dir_sizes.sort();

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", dir_sizes[0]);
}

fn parse(input: String) -> Vec<Command> {
    let lines: Vec<TerminalLine> = input.split("\n")
        .filter(|l| l != &"")
        .map(|l| TerminalLine::from_line(l))
        .collect();

    let mut terminal_history = Vec::<Command>::new();

    for line in lines {
        match line {
            TerminalLine::Command(cmd) => {
                match cmd {
                    CommandLine::Cd { argument } => {
                        terminal_history.push(Command::Cd {
                            argument: argument.clone()
                        })
                    },
                    CommandLine::Ls => {
                        terminal_history.push(Command::Ls {
                            output: vec![] 
                        })
                    }
                }
            },
            TerminalLine::Output(output_line) => {
                let len = terminal_history.len();
                let last_command = &mut terminal_history[len-1];

                match last_command {
                    Command::Ls { output } => {
                        output.push(output_line);
                    },
                    _ => panic!("Output for cd command")
                }
            }
        }
    }
    
    terminal_history
}
