fn main() {
    let lines: Vec<&str> = include_str!("./input.txt").lines().collect();

    // loop through lines and setup stacks and instructions vectors
    let mut stack_lines = Vec::new();
    let mut instructions = Vec::new();

    for line in lines {
        if line.len() > 1 && !line.starts_with("move") {
            stack_lines.push(line.clone());
        }
        if line.len() > 1 && line.starts_with("move") {
            instructions.push(line.clone());
        }
    }
    stack_lines.reverse();

    // determine number of stacks
    let n_stacks = (stack_lines[0].len() + 1)/4;
    println!("number of stacks: {}", n_stacks);

    // put stack_lines into vectors
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for i in 0..n_stacks {
        let mut v: Vec<char> = Vec::new();
        for stack_line in &stack_lines {
            let c = stack_line.chars().nth(i*4+1).unwrap().clone();
            if c != ' ' {
                v.push(c);
            }
        }
        stacks.push(v.clone());
    }
    
    println!("original stacks:");
    for i in 0..n_stacks {
        println!("{:?}", stacks[i]);
    }

    // reorder stacks
    for instruction in instructions {
        let (qty, from, to) = get_instructions(instruction);
        for _i in 0..qty {
            let c: char = stacks[from].pop().unwrap();
            stacks[to].push(c);
        }
    }

    println!("updated stacks:");
    let mut msg = String::from("");
    for i in 0..n_stacks {
        println!("{:?}", stacks[i]);
        msg.push(stacks[i].pop().unwrap());
    }
    println!("msg: {}", msg);

}

fn get_instructions(inst: &str) -> (i32, usize, usize) {
    let split: Vec<&str> = inst.split(" ").collect();
    let qty: i32 = split[1].parse::<i32>().unwrap();
    let from: usize = split[3].parse::<usize>().unwrap()-1;
    let to: usize = split[5].parse::<usize>().unwrap()-1;

    (qty, from, to)
}
