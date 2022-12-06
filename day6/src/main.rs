fn main() {
    let buffer: &str = include_str!("./input.txt");
    let n = sop_marker_pos(buffer);
    println!("pos: {}", n);
}

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

