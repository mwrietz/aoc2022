/*
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
*/

fn main() {
    /*
    let path = Path::new("input.txt"); 
    let mut lines = Vec::new();
    read_file_to_vector(&path, &mut lines);
    */
    let lines: Vec<&str> = include_str!("input.txt")
        .lines()
        .collect();

    let mut elf_sum: i32 = 0;
    let mut elf_cals: Vec<i32> = Vec::new();
    for calstring in lines {
        if calstring.len() > 0 {
            elf_sum += calstring.parse::<i32>().unwrap();
        }
        else {
            elf_cals.push(elf_sum);
            elf_sum = 0;
        }
    }
    elf_cals.push(elf_sum);

    let mut elf_cals2 = elf_cals.clone();
    let max_cal = elf_cals.iter().max().unwrap();
    println!("max_cal: {:?}", max_cal);

    elf_cals2.sort();
    println!("{:?}", elf_cals2);
    let n = elf_cals2.len();
    let max_cal_top_three = elf_cals2[n-1] + elf_cals2[n-2] + elf_cals2[n-3];
    println!("max top three: {}", max_cal_top_three);
}

/*
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
*/
