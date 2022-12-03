use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let path = Path::new("input.txt"); 
    let mut rucksacks = Vec::new();
    read_file_to_vector(&path, &mut rucksacks);

    let mut priority_sum = 0;
    for rs in &rucksacks {
        let rs_items: Vec<char> = rs.chars().collect();
        let n = rs_items.len();
        let rs_compartments: Vec<&[char]> =rs_items.chunks(n/2).collect();

        priority_sum += priority(duplicate_item(rs_compartments[0], rs_compartments[1]))
    }
    println!("part 1");
    println!("priority_sum: {}", priority_sum);
}

fn priority(item: char) -> i32 {
    let p = item as i32;
    if p>=97 && p<=122 {
        return p-96;
    }
    if p>=65 && p<=90 {
        return p-38;
    }
    0
}

fn duplicate_item(a1: &[char], a2: &[char]) -> char {
    for c in a1 {
        if a2.contains(c) {
            return *c;
        }
    }
    '_'
}

fn read_file_to_vector(file_path: &Path, vector: &mut Vec<String>) {
    let file = File::open(file_path).expect("cannot open filepath");
    let lines = io::BufReader::new(file).lines();

    for line in lines {
        if let Ok(ip) = line {
            vector.push(ip);
        }
    }
}
