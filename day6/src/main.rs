fn main() {
    let input = include_str!("input.txt");
    let mut count = 0;
    let mut count2 = 0;
    let mut acc = Vec::new();
    for line in input.lines() {
        let line = line.trim();

        if line.is_empty() {
            count += get_any_yes(&acc);
            count2 += get_all_yes(&acc);
            acc.clear();
        } else {
            acc.push(line);
        }
    }

    if acc.is_empty() == false {
        count += get_any_yes(&acc);
        count2 += get_all_yes(&acc);
        acc.clear();
    }

    println!("part1: {}", count);
    println!("part2: {}", count2);
}

fn get_any_yes(answers: &[&str]) -> i32 {
    let mut count = 0;
    let mut answered_bit = 0;
    for &answer in answers {
        for ch in answer.chars() {
            let index = ch as i32 - 'a' as i32;
            let cur_bit = 1 << index;
            if cur_bit & answered_bit == 0 {
                answered_bit |= cur_bit;
                count += 1;
            }
        }
    }

    count
}

fn get_all_yes(answers: &[&str]) -> i32 {
    let mut answered_bit = 0xffffffff_u32;
    for &answer in answers {

        let mut cur_answer = 0_u32;
        for ch in answer.chars() {
            let index = ch as u32 - 'a' as u32;
            let cur_bit = 1 << index;
            cur_answer |= cur_bit;
        }

        answered_bit &= cur_answer;
    }

    let mut count = 0;
    for i in 0..32 {
        let check_bit = 1_u32 << i;
        if answered_bit & check_bit == check_bit {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_any_yes() {
        assert_eq!(get_any_yes(&["abc"]), 3);
        assert_eq!(get_any_yes(&["a", "b", "c"]), 3);
        assert_eq!(get_any_yes(&["ab", "ac"]), 3);
        assert_eq!(get_any_yes(&["a", "a", "a", "a"]), 1);
    }

    #[test]
    fn test_get_all_yes() {
        assert_eq!(get_all_yes(&["abc"]), 3);
        assert_eq!(get_all_yes(&["a", "b", "c"]), 0);
        assert_eq!(get_all_yes(&["ab", "ac"]), 1);
        assert_eq!(get_all_yes(&["a", "a", "a", "a"]), 1);
    }
}
