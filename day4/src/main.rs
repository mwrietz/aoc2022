fn main() {
    let lines: Vec<&str> = include_str!("./input.txt").lines().collect();

    // part 1
    let mut n_fully_contained = 0;
    for line in lines {
        let assignments: Vec<&str> = line.split(",").collect(); 
        if contained(&assignments) {
            n_fully_contained += 1;
        }
    }
    println!("fully contained: {}", n_fully_contained);

    // part 2
}

fn contained(a: &Vec<&str>) -> bool {
    let (lb0, ub0) = bounds(a[0]);
    let (lb1, ub1) = bounds(a[1]);
    if lb0 >= lb1 && ub0 <= ub1 {
        return true;
    }
    if lb1 >= lb0 && ub1 <= ub0 {
        return true;
    }
    false
}

fn bounds(pair: &str) -> (i32, i32) {
    let s: Vec<&str> = pair.split("-").collect();
    let lb: i32 = s[0].parse::<i32>().unwrap();
    let ub: i32 = s[1].parse::<i32>().unwrap();
    (lb, ub)
}
