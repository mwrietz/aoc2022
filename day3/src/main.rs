fn main() {

    // part 1
    let rucksacks: Vec<&str> = include_str!("input.txt")
        .lines()
        .collect();

    let mut priority_sum = 0;
    for rs in rucksacks {
        let rs_items: Vec<char> = rs.chars().collect();
        let n = rs_items.len();
        let rs_compartments: Vec<&[char]> = rs_items.chunks(n/2).collect();

        priority_sum += priority(get_duplicate_item(rs_compartments[0], rs_compartments[1]))
    }
    println!("part 1");
    println!("priority_sum: {}", priority_sum);

    // part 2
    let rucksacks: Vec<&str> = include_str!("input.txt")
        .lines()
        .collect();

    let groups: Vec<&[&str]> = rucksacks.chunks(3).collect();

    let mut group_sum = 0;
    for group in groups {
        let first_string = &group[0];
        let first_chars: Vec<char> = first_string.chars().collect();
        for c in first_chars {
            if group[1].contains(c) && group[2].contains(c) {
                group_sum += priority(c);
                break;
            };
        }
    }
    println!("part 2");
    println!("priority_sum: {}", group_sum);
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

fn get_duplicate_item(a1: &[char], a2: &[char]) -> char {
    for c in a1 {
        if a2.contains(c) {
            return *c;
        }
    }
    '_'
}
