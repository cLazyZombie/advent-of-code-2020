fn main() {
    println!("Hello, world!");
}

fn get_first_invalid(preamble: usize, numbers: &[i32]) -> i32 {
    for i in preamble..numbers.len() {
        if is_valid(i, preamble, numbers) == false {
            return numbers[i];
        }
    }

    panic!("should have invalid number")
}

fn is_valid(idx: usize, preamble: usize, numbers: &[i32]) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        let numbers = [
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];

        assert_eq!(is_valid(0, 5, &numbers), true);
        assert_eq!(is_valid(4, 5, &numbers), true);
        assert_eq!(is_valid(5, 5, &numbers), true);
        assert_eq!(is_valid(14, 5, &numbers), false);
        assert_eq!(is_valid(15, 5, &numbers), true);
    }
}
