fn main() {
    let buffer1: &str = include_str!("./input_test_1.txt");
    let n1 = sop_marker_pos(buffer1);
    println!("pos: {}", n1);

    let buffer2: &str = include_str!("./input_test_2.txt");
    let n2 = sop_marker_pos(buffer2);
    println!("pos: {}", n2);

    let buffer3: &str = include_str!("./input_test_3.txt");
    let n3 = sop_marker_pos(buffer3);
    println!("pos: {}", n3);

    let buffer4: &str = include_str!("./input_test_4.txt");
    let n4 = sop_marker_pos(buffer4);
    println!("pos: {}", n4);

    let buffer5: &str = include_str!("./input_test_5.txt");
    let n5 = sop_marker_pos(buffer5);
    println!("pos: {}", n5);

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

