fn main() {
    let buffer: &str = include_str!("./input.txt");
    println!("part1 pos: {}", marker_pos(buffer, 4));
    println!("part2 pos: {}", marker_pos(buffer, 14));
}

fn marker_pos(buf: &str, marker_size: usize) -> usize {
    let v: Vec<char> = buf.chars().collect();
    let mut pos: usize = marker_size;
    loop {
        let mut marker: Vec<char> = Vec::new();
        for i in 0..marker_size {
            marker.push(v[pos - marker_size + i]);
        }
        let mut uniq = marker.clone();
        uniq.sort();
        uniq.dedup();
        if uniq.len() == marker.len() {
            break;
        }
        pos += 1;
    }

    pos
}
