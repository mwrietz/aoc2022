struct Position {
    row: usize,
    col: usize 
}

fn main() {
    let trees = parse_input();
    let w = trees[0].len();
    let h = trees.len();
    let mut visible = vec![vec![0; w]; h];

    set_perimeter_visibility(&mut visible);
    update_interior_visiblity(&trees, &mut visible);

    println!("trees:");
    print_vectors(&trees);

    println!("visibility:");
    print_vectors(&visible);

    println!("visible: {}", count_visible(&visible));

}

fn count_visible(v: &Vec<Vec<i32>>) -> i32 {
    let mut count = 0;

    for i in 0..v.len() {
        count += v[i].iter().sum::<i32>();
    }

    count
}

fn update_interior_visiblity(t: &Vec<Vec<i32>>, v: &mut Vec<Vec<i32>>) {
    let mut pos = Position {
        row: 0,
        col: 0,
    };
    let w = t[0].len();
    let h = t.len();

    for col in 1..(w - 1) {
        for row in 1..(h - 1) {
            pos.row = row;
            pos.col = col;
            if visible_left(t, &pos) {
                v[pos.row][pos.col] = 1;
            }
            if visible_right(t, &pos) {
                v[pos.row][pos.col] = 1;
            }
            if visible_top(t, &pos) {
                v[pos.row][pos.col] = 1;
            }
            if visible_bottom(t, &pos) {
                v[pos.row][pos.col] = 1;
            }
        }
    }
}

fn visible_top(t: &Vec<Vec<i32>>, p: &Position) -> bool {
    let mut visible = true;
    for row in 0..(p.row) {
        if t[row][p.col] >= t[p.row][p.col] {
            visible = false;
            break;
        }
    }

    visible
}

fn visible_bottom(t: &Vec<Vec<i32>>, p: &Position) -> bool {
    let h = t.len();
    let mut visible = true;
    for row in (p.row + 1)..h {
        if t[row][p.col] >= t[p.row][p.col] {
            visible = false;
            break;
        }
    }

    visible
}

fn visible_left(t: &Vec<Vec<i32>>, p: &Position) -> bool {
    let mut visible = true;
    for col in 0..(p.col) {
        if t[p.row][col] >= t[p.row][p.col] {
            visible = false;
            break;
        }
    }

    visible
}

fn visible_right(t: &Vec<Vec<i32>>, p: &Position) -> bool {
    let w = t[0].len();
    let mut visible = true;
    for col in (p.col + 1)..w {
        if t[p.row][col] >= t[p.row][p.col] {
            visible = false;
            break;
        }
    }

    visible
}

fn parse_input() -> Vec<Vec<i32>> {
    let lines: Vec<&str> = include_str!("./input.txt").lines().collect();

    let mut values: Vec<Vec<i32>> = Vec::new();
    for line in lines {
        let mut row_vals = Vec::new();
        for c in line.chars() {
            let v = c.to_string().parse::<i32>().unwrap();
            row_vals.push(v);
        }
        values.push(row_vals);
    }

    values
}

fn print_vectors(v: &Vec<Vec<i32>>) {
    for l in v {
        println!("{:?}", l);
    }
}

fn set_perimeter_visibility(v: &mut Vec<Vec<i32>>) {
    let w = v[0].len();
    let h = v.len();
    for i in 0..h {
        for j in 0..w {
            if i == 0 || i == (h - 1) {
                v[i][j] = 1;
            }
            else {
                if j == 0 || j == (h-1) {
                    v[i][j] = 1;
                }
            }
        }
    }
}
