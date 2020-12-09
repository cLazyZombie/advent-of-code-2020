fn main() {
    let input = include_str!("input.txt");

    // create color table
    let mut color_table: Vec<String> = Vec::new();
    let mut color_matrix_str: Vec<(String, Vec<(String, i32)>)> = Vec::new();

    for line in input.lines() {
        let (color, sub_colors) = parse_line(line);

        if color_table.contains(&color) == false {
            color_table.push(color.clone());
            color_table.sort();
        }

        let mut vec_colors = Vec::new();
        for sub_color in sub_colors {
            if color_table.contains(&sub_color.0) == false {
                color_table.push(sub_color.0.clone());
                color_table.sort();
            }

            vec_colors.push(sub_color);
        }

        color_matrix_str.push((color, vec_colors));
    }

    //println!("color: {},  {:?}", color_table.len(), color_table);

    // create matrix
    let mut color_matrix = vec![Vec::new();color_table.len()];
    for m in &mut color_matrix {
        *m = vec![0;color_table.len()];
    }

    for (color, sub_colors) in color_matrix_str {
        let color_idx = color_table.binary_search(&color).unwrap();
        color_matrix[color_idx][color_idx] = 1;

        for sub_color in sub_colors {
            let sub_color_idx = color_table.binary_search(&sub_color.0).unwrap();
            color_matrix[color_idx][sub_color_idx] = sub_color.1;
        }
    }

    let color_matrix_composed = compose_matrix(&mut color_matrix);

    // 
    let shiny_gold_idx = color_table.binary_search(&"shiny gold".to_string()).unwrap();
    let count = get_count_from_matrix(shiny_gold_idx, &color_matrix_composed);
    println!("part1: {}", count);

    let mut memo = vec![None;color_matrix.len()];
    let total_count = get_total_count(shiny_gold_idx, &color_matrix, &mut memo);
    println!("part2: {}", total_count);
}

fn parse_line(line: &str) -> (String, Vec<(String, i32)>) {
    let mut sentences = line.split(",").collect::<Vec<_>>();

    // special care first sentence ([color] bag contain [sentence])
    let first_sentence = sentences[0].split("contain").collect::<Vec<_>>();
    sentences[0] = first_sentence[1];

    let first_bag_index = line.find("bag").unwrap();
    let color = line[..first_bag_index].trim().to_string();

    let mut colors = Vec::new();
    for sentence in sentences {
        if let Some(sub) = parse_sentence(sentence) {
            colors.push(sub);
        }
    }

    (color, colors)
}

fn parse_sentence(sentence: &str) -> Option<(String, i32)> {
    let sentence = sentence.trim();
    let tokens = sentence.split_whitespace().collect::<Vec<_>>();

    let count;

    match tokens[0] {
        "no" => {
            return None;
        },
        t => {
            if let Ok(t) = t.parse::<i32>() {
                count = t;
            } else {
                return None;
            }
        }
    }

    // remove bag

    let mut color = String::new();
    for &tok in &tokens[1..tokens.len()-1] {
        if color.is_empty() == false {
            color.push(' ');
        }

        color.push_str(tok);
    }

    Some((color, count))
}

// 최적화 가능. DP 사용
fn compose_matrix(m: &Vec<Vec<i32>>) -> Vec<Vec<bool>>{
    let mut composed = vec![Vec::new(); m.len()];
    for i in 0..m.len() {
        composed[i] = vec![false; m.len()];
    }

    for row in 0..m.len() {
        for column in 0..m.len() {
            if m[row][column] > 0 {
                composed[row][column] = true;    
            }
        }
    }

    for i in 0..m.len() {
        let mut i_x = 0_usize;
        while i_x < m.len() {
            if i == i_x {
                i_x += 1;
                continue;
            }

            let mut modified = false;
            if composed[i][i_x] == true {
                let j = i_x;
                for j_x in 0..m.len() {
                    if composed[j][j_x] == true && composed[i][j_x] == false {
                        composed[i][j_x] = true;
                        modified = true;
                    }
                }
            }

            if modified {
                i_x = 0;
            } else {
                i_x += 1;
            }
        }
    }

    composed
}

fn get_count_from_matrix(idx: usize, m: &Vec<Vec<bool>>) -> i32 {
    let mut count = 0;
    for row in 0..m.len() {
        if row == idx {
            continue;
        }

        if m[row][idx] {
            count += 1;
        }
    }

    count
}

fn get_total_count(idx: usize, m: &Vec<Vec<i32>>, memo: &mut Vec<Option<i32>>) -> i32 {
    if let Some(cached) = memo[idx] {
        return cached;
    }

    let mut total_count = 0;

    for i in 0..m.len() {
        if i == idx {
            continue;
        }

        let count = m[idx][i];
        if count > 0 {
            total_count += count * (get_total_count(i, m, memo) + 1);
        }
    }

    memo[idx] = Some(total_count);
    return total_count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let (color, contains) = parse_line("light red bags contain 1 bright white bag, 2 muted yellow bags.");
        assert_eq!(color, "light red");
        assert_eq!(contains.len(), 2);
        assert_eq!(contains[0], ("bright white".to_string(), 1));
        assert_eq!(contains[1], ("muted yellow".to_string(), 2));


        let (color, contains) = parse_line("dotted coral bags contain 4 dotted chartreuse bags, 2 bright red bags, 1 vibrant white bag, 1 vibrant brown bag.");
        assert_eq!(color, "dotted coral");
        assert_eq!(contains.len(), 4);
        assert_eq!(contains[0], ("dotted chartreuse".to_string(), 4));
        assert_eq!(contains[1], ("bright red".to_string(), 2));
        assert_eq!(contains[2], ("vibrant white".to_string(), 1));
        assert_eq!(contains[3], ("vibrant brown".to_string(), 1));

        let (color, contains) = parse_line("dotted black bags contain no other bags.");
        assert_eq!(color, "dotted black");
        assert_eq!(contains.len(), 0);
    }

    #[test]
    fn test_parse_sentence() {
        assert_eq!(parse_sentence("1 bright white bag"), Some(("bright white".to_string(), 1)));
        assert_eq!(parse_sentence(" 2 muted yellow bags."), Some(("muted yellow".to_string(), 2)));
    }

    #[test]
    fn test_total_count() {
        let m = vec![
            vec![1, 0, 0, 1],
            vec![0, 1, 0, 0],
            vec![0, 3, 1, 0],
            vec![0, 0, 2, 1],
        ];

        // 0번째 -> 4번째 1개 -> 3번째 2개 -> 2번째 3개 = 1 + 2 * (1 + 3) = 9개
        let mut memo = vec![None;m.len()];
        assert_eq!(get_total_count(0, &m, &mut memo), 9);
    }

    #[test]
    fn test_compose_matrix() {
        let mut m = vec![
            vec![1, 0, 3],
            vec![2, 1, 0],
            vec![0, 0, 1],
        ];

        let composed = compose_matrix(&mut m);

        let expected = vec![
            vec![true, false, true],
            vec![true, true, true],
            vec![false, false, true],
        ];

        assert_eq!(composed, expected);

        // 2단계 이상 case
        let mut m = vec![
            vec![1, 0, 0, 1],
            vec![0, 1, 0, 0],
            vec![0, 1, 1, 0],
            vec![0, 0, 1, 1],
        ];

        let composed = compose_matrix(&mut m);

        let expected = vec![
            vec![true, true, true, true],
            vec![false, true, false, false],
            vec![false, true, true, false],
            vec![false, true, true, true],
        ];

        assert_eq!(composed, expected); 


        let mut m = vec![
            vec![1, 0, 0, 1],
            vec![1, 1, 0, 0],
            vec![0, 1, 1, 0],
            vec![0, 0, 1, 1],
        ];

        let composed = compose_matrix(&mut m);

        let expected = vec![
            vec![true, true, true, true],
            vec![true, true, true, true],
            vec![true, true, true, true],
            vec![true, true, true, true],
        ];

        assert_eq!(composed, expected); 
    }
}