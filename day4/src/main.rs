use std::fs::read_to_string as read;

#[derive(Clone, Copy)]
struct Range {
    start: usize,
    end: usize
}

impl Range {
    pub fn contains_range(&self, other: Range) -> bool {
        return self.start <= other.start && self.end >= other.end
    }

    pub fn overlaps_with(&self, other: Range) -> bool {
        self.start <= other.end && self.end >= other.start 
    }

    fn new(start: usize, end: usize) -> Range {
        Range { start, end }
    }

    pub fn from_str(string: &str) -> Range {
        let mut split = string.split("-");
        let start = split.next().unwrap().parse::<usize>().unwrap();
        let end = split.next().unwrap().parse::<usize>().unwrap();

        Range::new(start, end)    
    }
}

fn main() {
    let content = read("input.txt");
    let ranges: Vec<(Range, Range)> = content.unwrap().split("\n")
        .filter(|line| line != &"")
        .map(|line| line.strip_suffix("\n").unwrap_or(line))
        .map(|line| {
            let mut split = line.split(",");
            let first = Range::from_str(split.next().unwrap());
            let second = Range::from_str(split.next().unwrap());
            (first, second) 
        })
        .collect();

    let contains_count: usize = ranges.iter().map(|range_pair| {
        if range_pair.0.contains_range(range_pair.1) ||
        range_pair.1.contains_range(range_pair.0)
            { 1 } else { 0 }
    }).sum();

    let overlaps_count: usize = ranges.iter().map(|range_pair| {
        if range_pair.0.overlaps_with(range_pair.1) { 1 } else { 0 }
    }).sum();

    println!("Contains: {}", contains_count);
    println!("Overlaps: {}", overlaps_count);
    
}
