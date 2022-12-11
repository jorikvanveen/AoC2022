use std::fs::read_to_string as read;

fn main() {
    let signal: String = read("input.txt").unwrap();

    for i in 0..(signal.len()-4) {
        let slice = &signal[i..i+4];

        if is_marker(slice) {
            println!("Part 1: {}", i+4);
            break;  
        }
    }

    for i in 0..(signal.len()-14) {
        let slice = &signal[i..i+14];

        if is_marker(slice) {
            println!("Part 1: {}", i+14);
            break;  
        }
    }
}

fn is_marker(slice: &str) -> bool {
    let mut deduped = slice.chars().collect::<Vec<char>>();
    deduped.sort();
    deduped.dedup();
    deduped.len() == slice.len()
}
