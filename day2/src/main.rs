fn main() {
    let input = include_str!("input.txt");

    let mut valid_count = 0;
    let mut valid_count2 = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.len() == 0 {
            break;
        }

        let v = line.split_whitespace().collect::<Vec<_>>();

        //num-num -> start, end
        let num_num = v[0].split('-').collect::<Vec<_>>();
        let start = num_num[0].parse::<i32>().unwrap();
        let end = num_num[1].parse::<i32>().unwrap();

        // c: -> c
        let ch = v[1].chars().collect::<Vec<_>>()[0];

        // pw
        let lw = v[2];

        // part 1
        let mut count = 0;
        for c in lw.chars() {
            if c == ch {
                count += 1;
            }
        }

        if count >= start && count <= end {
            valid_count += 1;
        }

        // part 2
        let start_char = lw.chars().nth(start as usize - 1);
        let end_char = lw.chars().nth(end as usize - 1);

        let start_valid = start_char.is_some() && start_char.unwrap() == ch;
        let end_valid = end_char.is_some() && end_char.unwrap() == ch;

        if (start_valid && !end_valid) || (!start_valid && end_valid) {
            valid_count2 += 1;
        }
    }

    println!("part1: {}", valid_count);
    println!("part2: {}", valid_count2);
}
