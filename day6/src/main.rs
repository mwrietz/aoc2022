fn main() {
    //part 1
    let buffer: &str = include_str!("./input.txt");
    let n = sop_marker_pos(buffer);
    println!("pos: {}", n);
    //part 2
    let part2buf: &str = include_str!("./input.txt");
    let n2 = som_marker_pos(part2buf);
    println!("pos: {}", n2);
}

// part 1
fn sop_marker_pos(buf: &str) -> usize {
    let v: Vec<char> = buf.chars().collect();
    let mut pos: usize = 4;
    loop {
        let mut sop_marker: Vec<char> = Vec::new();
        for i in 0..4 {
            sop_marker.push(v[pos - 4 + i]);
        }
        let mut uniq = sop_marker.clone();
        uniq.sort();
        uniq.dedup();
        if uniq.len() == sop_marker.len() {
            break;
        }
        pos += 1;
    }

    pos
}

// part 2
fn som_marker_pos(buf: &str) -> usize {
    let v: Vec<char> = buf.chars().collect();
    let mut pos: usize = 14;
    loop {
        let mut som_marker: Vec<char> = Vec::new();
        for i in 0..14 {
            som_marker.push(v[pos - 14 + i]);
        }
        let mut uniq = som_marker.clone();
        uniq.sort();
        uniq.dedup();
        if uniq.len() == som_marker.len() {
            break;
        }
        pos += 1;
    }

    pos
}

