use std::fs::read_to_string as read;
use std::cmp::Ordering;

#[derive(Debug)]
struct Elf {
    calories: u32
}

impl Elf {
    fn default() -> Elf {
        Elf { calories: 0 }
    }

    fn cmp_calories(&self, other: &Elf) -> Ordering {
        if self.calories > other.calories {
            Ordering::Greater
        } else if self.calories < other.calories {
            Ordering::Less
        } else { Ordering::Equal }
    }
}

fn main() {
    let input = read("input/input.txt").unwrap();
    let lines = input.split("\n")
        .map(|s| s.trim())
        //.filter(|s| *s != "")
        .collect::<Vec<&str>>();

    let mut elves: Vec<Elf> = vec![Elf::default()];

    for line in lines.iter() {
        let last_idx = elves.len()-1;
        let last_elf = &mut elves[last_idx];  

        if line == &"" {
            elves.push(Elf::default());
            continue
        } 

        let calories = line.parse::<u32>().unwrap();
        last_elf.calories += calories;

    }

    // Get elf with most calories
    let mut fattest_elf_idx: Option<usize> = None;

    for (i, elf) in elves.iter().enumerate() {
        match fattest_elf_idx {
            Some(fattest_elf_idx_deref) => {
                let fattest_elf = &elves[fattest_elf_idx_deref];

                if elf.calories > fattest_elf.calories {
                    fattest_elf_idx = Some(i);
                }
            },
            None => fattest_elf_idx = Some(i)
        }
    }


    elves.sort_by(|a, b| a.cmp_calories(b));
    elves.reverse();
    let total = elves[0].calories + elves[1].calories + elves[2].calories;

    println!("{:#?}", total);
}
