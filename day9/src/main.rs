fn main() {
    let input = include_str!("input.txt");

    let mut numbers = Vec::new();
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        numbers.push(line.parse::<i64>().unwrap());
    }

    let part1 = get_first_invalid(25, &numbers);
    println!("part1: {}", part1);

    let part2 = continious_set(part1, &numbers);
    println!("part2: {}", part2);
}

fn get_first_invalid(preamble: usize, numbers: &[i64]) -> i64 {
    for i in preamble..numbers.len() {
        if is_valid(i, preamble, numbers) == false {
            return numbers[i];
        }
    }

    panic!("should have invalid number")
}

fn is_valid(idx: usize, preamble: usize, numbers: &[i64]) -> bool {
    if idx < preamble {
        return true;
    }

    let target = numbers[idx];

    for i in (idx - preamble)..idx {
        for j in (idx - preamble)..idx {
            if i == j {
                continue;
            }

            if numbers[i] + numbers[j] == target {
                return true;
            }
        }
    }

    return false;
}

fn continious_set(target: i64, numbers: &[i64]) -> i64 {
    for i in 0..(numbers.len()-1) {
        let mut smallest = numbers[i];
        let mut largest = numbers[i];

        let mut sum = numbers[i];
        for j in (i+1)..numbers.len() {
            sum += numbers[j];
            smallest = if numbers[j] < smallest { numbers[j] } else { smallest };
            largest = if numbers[j] > largest { numbers[j] } else { largest };

            if sum == target {
                return smallest + largest;
            } else if sum > target {
                break;
            }
        }
    }

    panic!("non solution");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        let numbers = [ 35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576 ];

        assert_eq!(is_valid(0, 5, &numbers), true);
        assert_eq!(is_valid(4, 5, &numbers), true);
        assert_eq!(is_valid(5, 5, &numbers), true);
        assert_eq!(is_valid(14, 5, &numbers), false);
        assert_eq!(is_valid(15, 5, &numbers), true);
    }

    #[test]
    fn test_continious_set() {
        let numbers = [ 35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576 ];

        assert_eq!(continious_set(127, &numbers), 62);
    }
}
