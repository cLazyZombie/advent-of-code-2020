fn main() {
    let input = include_str!("input.txt");

    let mut values = Vec::new();

    for line in input.lines() {
        let line = line.trim();
        if line.len() == 0 {
            break;
        }
        values.push(line.parse::<i64>().unwrap());
    }

    'part1: for i in 0..values.len() - 1 {
        for j in i + 1..values.len() {
            if values[i] + values[j] == 2020 {
                println!(
                    "part1. {} * {} = {}",
                    values[i],
                    values[j],
                    values[i] * values[j]
                );
                break 'part1;
            }
        }
    }

    'part2: for i in 0..(values.len() - 2) {
        for j in (i + 1)..(values.len() - 1) {
            for k in (j + 1)..(values.len()) {
                if values[i] + values[j] + values[k] == 2020 {
                    println!(
                        "part2. {} * {} * {} = {}",
                        values[i],
                        values[j],
                        values[k],
                        values[i] * values[j] * values[k]
                    );
                    break 'part2;
                }
            }
        }
    }
}
