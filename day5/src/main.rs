fn main() {
    let input = include_str!("input.txt");

    let mut highest = -1;

    let mut seats = Vec::new();

    for line in input.lines() {
        let line = line.trim();
        if line.len() != 10 {
            continue;
        }
        let seat_number = get_seat_number(line);

        if seat_number > highest {
            highest = seat_number;
        }

        seats.push(seat_number);
    }

    println!("highest: {}", highest);

    seats.sort();

    for seat in 0..1024 {
        if seats.binary_search(&seat).is_ok() {
            continue;
        }

        if seats.binary_search(&(seat-1)).is_err() {
            continue;
        }

        if seats.binary_search(&(seat+1)).is_err() {
            continue;
        }

        if seat & 0b1111111000 == 0b1111111000 {
            continue;
        }

        if seat & 0b1111111000 == 0b0000000000 {
            continue;
        }

        println!("seat: {}", seat);
    }
}


fn get_seat_number(s : &str) -> i32 {
    let mut result = 0;
    for ch in s.chars() {
        result <<= 1;

        match ch {
            'B' | 'R' => {
                result += 1;
            }
            _ => {}
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_seat_number() {
        assert_eq!(get_seat_number("FBFBBFFRLR"), 357);
    }
}