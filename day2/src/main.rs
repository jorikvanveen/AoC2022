use std::fs::read_to_string as read;

enum Move {
    Rock,
    Paper,
    Scissors
}

impl Move {
    pub fn from_str(string: &str) -> Move {
        match string {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            "X" => Move::Rock,
            "Y" => Move::Paper,
            "Z" => Move::Scissors,
            _ => panic!("Invalid move {}", string)
        }
    }

    pub fn get_score(&self) -> usize {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3
        }
    }

    pub fn into_outcome(&self) -> Outcome {
        match self {
            Move::Rock => Outcome::Loss,
            Move::Paper => Outcome::Draw,
            Move::Scissors => Outcome::Win
        }
    }
}

enum Outcome {
    Win,
    Loss,
    Draw
}

impl Outcome {
    pub fn get_score(&self) -> usize {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0
        }
    }
}

struct Round {
    move1: Move,
    move2: Move
}

impl Round {
    pub fn from_str(string: &str) -> Round {
        let moves_str: Vec<&str> = string.split(" ").collect();
        Round {
            move1: Move::from_str(moves_str[0]),
            move2: Move::from_str(moves_str[1])
        }
    }
    
    pub fn get_outcome(&self) -> Outcome {
        match self.move1 {
            Move::Rock => match self.move2 {
                Move::Rock => Outcome::Draw,
                Move::Paper => Outcome::Win,
                Move::Scissors => Outcome::Loss
            },
            Move::Paper => match self.move2 {
                Move::Rock => Outcome::Loss,
                Move::Paper => Outcome::Draw,
                Move::Scissors => Outcome::Win
            },
            Move::Scissors => match self.move2 {
                Move::Rock => Outcome::Win,
                Move::Paper => Outcome::Loss,
                Move::Scissors => Outcome::Draw
            }
        }
    }

    pub fn get_move(&self) -> Move {
        match (&self.move1, self.move2.into_outcome()) {
            (Move::Rock, Outcome::Loss) => Move::Scissors,
            (Move::Rock, Outcome::Draw) => Move::Rock,
            (Move::Rock, Outcome::Win) => Move::Paper,
            (Move::Paper, Outcome::Loss) => Move::Rock,
            (Move::Paper, Outcome::Draw) => Move::Paper,
            (Move::Paper, Outcome::Win) => Move::Scissors,
            (Move::Scissors, Outcome::Loss) => Move::Paper,
            (Move::Scissors, Outcome::Draw) => Move::Scissors,
            (Move::Scissors, Outcome::Win) => Move::Rock,
        }
    }

    pub fn get_score(&self) -> usize {
        self.get_outcome().get_score() + self.move2.get_score()
    }
}

fn main() {
    let contents = read("input.txt").unwrap();
    let mut lines = contents.split("\n").collect::<Vec<&str>>();
    lines.pop();
    let rounds: Vec<Round> = lines.iter().map(|l| Round::from_str(l)).collect();

    let mut total_p1: usize = 0;

    for round in rounds.iter() {
        total_p1 += round.get_score();
    }

    println!("Part 1: {}", total_p1);

    let mut total_p2: usize = 0;

    for round in rounds.iter() {
        total_p2 += round.get_move().get_score() + round.move2.into_outcome().get_score();
    }

    println!("Part 2: {}", total_p2);


}
