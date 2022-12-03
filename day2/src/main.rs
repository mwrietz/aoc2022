use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let path = Path::new("input.txt"); 
    let mut rounds = Vec::new();
    read_file_to_vector(&path, &mut rounds);

    // part 1
    let mut my_score: i32 = 0;
    for round in &rounds {
        let elf_selection = round.chars().nth(0).unwrap();
        let my_selection = round.chars().nth(2).unwrap();
        my_score += score(elf_selection, my_selection);
    }
    println!("part 1 score: {}", my_score);

    // part 2
    my_score = 0;
    //let mut directive = ' ';
    for round in &rounds {
        let elf_selection = round.chars().nth(0).unwrap();
        let directive = round.chars().nth(2).unwrap();
        let my_selection = get_selection(elf_selection, directive);
        my_score += score(elf_selection, my_selection);
    }
    println!("part 2 score: {}", my_score);

}

fn score(elf: char, me: char) -> i32 {
    let score = match elf {
        'A' => match me {
            'X' => 3 + 1,
            'Y' => 6 + 2,
            'Z' => 0 + 3,
             _ => 0
        },
        'B' => match me {
            'X' => 0 + 1,
            'Y' => 3 + 2,
            'Z' => 6 + 3, 
             _ => 0
        },
        'C' => match me {
            'X' => 6 + 1,
            'Y' => 0 + 2,
            'Z' => 3 + 3,
             _ => 0
        },
        _ => 0
    };
    score
}

fn get_selection(elf_selection: char, directive: char) -> char{
    let selection = match directive {
        // lose
        'X' => match elf_selection {
            'A' => 'Z',
            'B' => 'X',
            'C' => 'Y',
            _ => ' '
        },
        // draw
        'Y' => match elf_selection {
            'A' => 'X',
            'B' => 'Y',
            'C' => 'Z',
            _ => ' '
        },
        // win
        'Z' => match elf_selection {
            'A' => 'Y',
            'B' => 'Z',
            'C' => 'X',
            _ => ' '
        },
        _ => ' '
    };
    selection
}

fn read_file_to_vector(file_path: &Path, vector: &mut Vec<String>) {
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                vector.push(ip);
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
