use std::fs::read_to_string as read;

#[derive(Clone, Copy, PartialEq, Eq)]
struct Item(char);

impl Item {
    pub fn from_char(c: char) -> Item {
        println!("{}", c);
        let mut unicode = vec![0]; 
        c.encode_utf8(&mut unicode);

        if unicode.len() > 1 {
            panic!("Invalid input, character has more than 1 unicode value");
        }

        let char_code = unicode[0];

        match char_code {
            i if i >= 65 && i <= 90 => Item(c), // Uppercase letter
            i if i >= 97 && i <= 122 => Item(c), // Lowercase letter
            _ => panic!("Invalid unicode character, not part of alphabet")
        }
    }

    pub fn get_priority(&self) -> usize {
        let c: char = self.0.to_lowercase().next().unwrap();
        let mut priority = 0;
        
        // If self.0 was converted to lowercase in the first line of this function
        if self.0.is_uppercase() { priority += 26; }

        let mut unicode = vec![0];
        c.encode_utf8(&mut unicode);

        priority += unicode[0] - 96;

        priority.into()
    }
}

#[derive(Clone)]
struct Rucksack {
    compartment1: Vec<Item>,
    compartment2: Vec<Item>
}

impl Rucksack {
    pub fn from_line(line: &str) -> Rucksack {
        // Split line into 2 parts
        let split_idx = line.len() / 2;
        let (first_part, second_part) = line.split_at(split_idx);

        println!("{} {}", first_part, second_part);

        let compartment1: Vec<Item> = first_part.chars().map(|c| Item::from_char(c)).collect();
        let compartment2: Vec<Item> = second_part.chars().map(|c| Item::from_char(c)).collect();
        
        Rucksack { compartment1, compartment2 }
    }

    pub fn get_duplicate(&self) -> Item {
        for item1 in self.compartment1.iter() {
            for item2 in self.compartment2.iter() {
                if item1 == item2 {
                    return *item1
                }
            }
        }

        panic!("No duplicate found");
    }

    pub fn all_items(&self) -> Vec<Item> {
        let mut all: Vec<Item> = vec![];

        for item in self.compartment1.iter() {
            all.push(*item);
        }

        for item in self.compartment2.iter() {
            all.push(*item);
        }

        all
    }
}

fn main() {
    let input = read("input.txt").unwrap();

    let mut lines: Vec<&str> = input.split("\n").collect();
    lines.pop();
    
    let rucksacks: Vec<Rucksack> = lines.iter().map(|line| {
        println!("{}", line);
        Rucksack::from_line(line)
    }).collect();
    let duplicates: Vec<Item> = rucksacks.iter().map(|r| r.get_duplicate()).collect();
    let sum_of_duplicates: usize = duplicates.iter().map(|i| i.get_priority())
                                    .sum();

    let mut sum_of_badges: usize = 0;
    for i in 0..(rucksacks.len()/3) {
        let team: Vec<Rucksack> = vec![
            rucksacks[i*3].clone(),
            rucksacks[i*3+1].clone(),
            rucksacks[i*3+2].clone()
        ];
        
        let mut badge: Option<Item> = None;

        for item in team[0].all_items() {
            if team[1].all_items().iter().find(|i| i == &&item).is_some() &&
               team[2].all_items().iter().find(|i| i == &&item).is_some() {
                 badge = Some(item);
                 break;
            };
        }

        sum_of_badges += badge.unwrap().get_priority(); 
    }

    println!("Part 1: {}", sum_of_duplicates);
    println!("Part 2: {}", sum_of_badges);
}
