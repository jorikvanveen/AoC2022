use std::fs::read_to_string as read;

#[derive(Debug)]
struct Crate {
    label: char
}

#[derive(Debug)]
struct Move {
    from: usize,
    to: usize,
    count: usize
}

impl Move {
    pub fn from_str(line: &str) -> Move {
        let split: Vec<&str> = line.trim().split(" ").collect();
        let count = split[1].parse::<usize>().unwrap();
        let from = split[3].parse::<usize>().unwrap() - 1;
        let to = split[5].parse::<usize>().unwrap() - 1;

        Move { from, to, count }
    }
}

#[derive(Debug)]
struct Stack {
    crates: Vec<Crate>
}

impl Stack {
    pub fn new() -> Stack {
        Stack { crates: vec![] }
    }

    pub fn reverse(&mut self) {
        self.crates.reverse();
    }

    pub fn push_crate(&mut self, c: Crate) {
        self.crates.push(c);
    }

    pub fn pop_crate(&mut self) -> Crate {
        self.crates.pop().unwrap()
    }

    pub fn get_top_label(&self) -> String {
        let top_crate = &self.crates[self.crates.len()-1];
        top_crate.label.to_string().to_owned()
    }
}

#[derive(Debug)]
struct Stacks {
    stacks: Vec<Stack>,
    moves: Vec<Move>
}

impl Stacks {
    pub fn new(stacks: Vec<Stack>, moves: Vec<Move>) -> Stacks {
        Stacks { stacks, moves }
    }

    pub fn evaluate_9000(&mut self) {
        for mv in self.moves.iter() {
            for _ in 0..mv.count {
                let cr = self.stacks[mv.from].pop_crate();
                self.stacks[mv.to].push_crate(cr);
            }
        }
    }

    pub fn evaluate_9001(&mut self) {
        for mv in self.moves.iter() {
            let mut moving = Vec::<Crate>::new();

            for _ in 0..mv.count {
                moving.push(self.stacks[mv.from].pop_crate())
            }

            moving.reverse();

            for cr in moving {
                self.stacks[mv.to].push_crate(cr)
            }
        }
    }

    pub fn get_message(&self) -> String {
        let mut msg = String::new();
        
        for stack in self.stacks.iter() {
            msg.push_str(&stack.get_top_label());
        }

        msg
    }
}

fn main() {
    let mut stacks = parse(read("input.txt").unwrap());
    stacks.evaluate_9000();
    println!("Part 1: {}", stacks.get_message());

    let mut stacks2 = parse(read("input.txt").unwrap());
    stacks2.evaluate_9001();
    println!("Part 2: {}", stacks2.get_message());

}

fn parse(content: String) -> Stacks {
    let lines: Vec<&str> = content.split("\n")
        .filter(|l| l != &"\n" && l != &"")
        .collect();

    let stack_count = (lines[0].len()+2) / 4;
    let mut stacks_vec = Vec::<Stack>::new();
    let mut moves_vec = Vec::<Move>::new();
    // Get maximum value in moves

    for _ in 0..stack_count {
        stacks_vec.push(Stack::new());
    }

    enum ParseStage {
        Stacks,
        Moves
    }

    let mut parse_stage = ParseStage::Stacks;

    // Populate stacks with crates and moves
    for line in lines.iter() {
        match parse_stage {
            ParseStage::Stacks => {
                if line.chars().nth(1) == Some('1') {
                    parse_stage = ParseStage::Moves;
                    continue;
                }

                for i in 0..stack_count {
                    let label_idx = i * 4 + 1;
                    let label = line.chars().nth(label_idx).unwrap();

                    if label == ' ' { continue }
                    let c = Crate { label };
                    stacks_vec[i].push_crate(c);
                }
            },

            ParseStage::Moves => {
                let mov = Move::from_str(line);
                moves_vec.push(mov);
            }
        }
    }

    // Reverse all the stacks (so that push and pop make sense)
    for stack in stacks_vec.iter_mut() {
        stack.reverse();
    }

    Stacks::new(stacks_vec, moves_vec)
}

