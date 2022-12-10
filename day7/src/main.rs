use std::collections::HashMap;

fn main() {
    let lines: Vec<&str> = include_str!("./input.txt").lines().collect();

    let mut size_hm = HashMap::new();
    size_hm.insert("/".to_string(), 0);

    let mut dir_stack = Vec::new();

    for line in lines {
        //let mut size = 0;
        if line.starts_with("$ cd") {
            let tokens: Vec<&str> = line.split(" ").collect();

            match tokens[2] {
                "/" => {
                    dir_stack.clear();
                    dir_stack.push(tokens[2]);
                }
                ".." => {
                    dir_stack.pop();
                }
                _ => {
                    dir_stack.push(tokens[2]);
                    size_hm.insert(key_from(&dir_stack), 0);
                }
            }
        }
        if line.starts_with("$ ls") {
            continue;
        }
        if line.chars().next().unwrap().is_numeric() {
            let tokens: Vec<&str> = line.split(" ").collect();
            let size = tokens[0].parse::<i32>().unwrap();
            update_size_hm(&mut size_hm, &dir_stack, size);
        }
    }
    println!("{:?}", &size_hm);
    println!("part a: {}", sum_lt_100000(&size_hm));
    println!("part b:");
    println!("/ size: {}", size_hm["/"]);
    println!("min size to delete: {}", min_size_to_delete(&size_hm));
}

fn min_size_to_delete(hm: &HashMap<String, i32>) -> i32 {
    let free = 70_000_000 - hm["/"];
    let mut size = hm["/"];
    for (_key, value) in hm.into_iter() {
        if free + value > 30_000_000 {
            if value < &size {
                size = *value;
            }
        }
    }
    size
}

fn sum_lt_100000(hm: &HashMap<String, i32>) -> i32 {
    let mut sum = 0;
    for (_key, value) in hm.into_iter() {
        if value < &100000 {
            sum += value;
        }
    }
    sum
}
fn update_size_hm(hm: &mut HashMap<String, i32>, stack: &Vec<&str>, size: i32) {
    let mut stk = stack.clone();
    while stk.len() > 0 {
        hm.insert(key_from(&stk), hm[&key_from(&stk)] + size);
        stk.pop();
    }
}

fn key_from(stack: &Vec<&str>) -> String {
    let s = stack.clone();
    let mut key = s.join("/");

    if key.starts_with("//") {
        key.remove(0);
    }

    key
}
