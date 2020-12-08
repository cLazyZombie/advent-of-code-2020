fn main() {
    let input = include_str!("input.txt");

    let mut w = 0;
    let mut h = 0;
    let mut map = Vec::new();
    for line in input.lines() {
        let line = line.trim();
        
        if line.len() > 0 {
            w = line.chars().count();
            h += 1_usize;
            map.push(line);
        }
    }

    let tree_count = get_tree_count(&map, w, h, 3, 1);

    let t1 = get_tree_count(&map, w, h, 1, 1);
    let t2 = get_tree_count(&map, w, h, 3, 1);
    let t3 = get_tree_count(&map, w, h, 5, 1);
    let t4 = get_tree_count(&map, w, h, 7, 1);
    let t5 = get_tree_count(&map, w, h, 1, 2);

    println!("part1: {}", tree_count);
    println!("part2: {}", t1 * t2 * t3 * t4 * t5);
}

fn get_tree_count(map: &Vec<&str>, w: usize, h: usize, inc_x: usize, inc_y: usize) -> usize {
    let mut tree_count = 0;
    let mut x = 0;
    let mut y = 0;
    while y < h {
        let geo = map[y].chars().nth(x).unwrap();
        if geo == '#' {
            tree_count += 1;
        }

        x += inc_x;
        if x >= w {
            x -= w;
        }

        y += inc_y;
    }

    tree_count
}
