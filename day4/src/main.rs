use regex::Regex;

fn main() {
    let input = include_str!("input.txt");

    let mut acc = String::new();
    let mut count = 0;
    let mut count2 = 0;
    for line in input.lines() {
        let line = line.trim();

        if line.is_empty() {
            if check_valid(&acc) {
                count += 1;
            }

            if check_valid2(&acc) {
                count2 += 1;
            }
            acc.clear();
        } else {
            acc.push_str(line);
            acc.push(' ');
        }
    }

    if acc.trim().len() > 0 {
        if check_valid(&acc) {
            count += 1;
        }

        if check_valid2(&acc) {
            count2 += 1;
        }

        acc.clear();
    }

    println!("part1: {}", count);
    println!("part2: {}", count2);
}

fn check_valid(input: &str) -> bool {
    let mut result = 0;
    for entity in input.split_whitespace() {
        let items = entity.split(':').collect::<Vec<_>>();
        match items[0] {
            "byr" => {
                result |= 0b00000001; 
            }
            "iyr" => { 
                result |= 0b00000010; 
            }
            "eyr" => { 
                result |= 0b00000100; 
            }
            "hgt" => { 
                result |= 0b00001000; 
            }
            "hcl" => { 
                result |= 0b00010000; 
            }
            "ecl" => { 
                result |= 0b00100000; 
            }
            "pid" => { 
                result |= 0b01000000; 
            }
            "cid" => { 
                result |= 0b10000000; 
            }
            _ => {}
        }
    }

    if result & 0b01111111 == 0b01111111 {
        return true;
    } else {
        return false;
    }
}

fn check_valid2(input: &str) -> bool {
    let mut result = 0;
    for entity in input.split_whitespace() {
        let items = entity.split(':').collect::<Vec<_>>();
        match items[0] {
            "byr" => {
                if items[1].len() == 4 {
                    if let Ok(birth) = items[1].parse::<i32>() {
                        if birth >= 1920 && birth <= 2002 {
                            result |= 0b00000001; 
                        }
                    }
                }
            }
            "iyr" => { 
                if items[1].len() == 4 {
                    if let Ok(issue) = items[1].parse::<i32>() {
                        if issue >= 2010 && issue <= 2020 {
                            result |= 0b00000010; 
                        }
                    }
                }
            }
            "eyr" => { 
                if items[1].len() == 4 {
                    if let Ok(expr) = items[1].parse::<i32>() {
                        if expr >= 2020 && expr <= 2030 {
                            result |= 0b00000100; 
                        }
                    }
                }
            }
            "hgt" => {
                if items[1].ends_with("cm") {
                    if let Ok(cm) = items[1][..items[1].len()-2].parse::<i32>() {
                        if cm >= 150 && cm <= 193 {
                            result |= 0b00001000;             
                        }
                    }
                }

                if items[1].ends_with("in") {
                    if let Ok(inch) = items[1][..items[1].len()-2].parse::<i32>() {
                        if inch >= 59 && inch <= 76 {
                            result |= 0b00001000;             
                        }
                    }
                }
            }
            "hcl" => { 
                if items[1].len() == 7 {
                    let re = Regex::new(r"#(([0-9]|[a-f]){6})").unwrap();
                    if re.is_match(items[1]) {
                        result |= 0b00010000; 
                    }
                }
            }
            "ecl" => { 
                let re = Regex::new(r"amb|blu|brn|gry|grn|hzl|oth").unwrap();
                if re.is_match(items[1]) {
                    result |= 0b00100000; 
                }
            }
            "pid" => { 
                if items[1].len() == 9 {
                    if let Ok(_) = items[1].parse::<i32>() {
                        result |= 0b01000000; 
                    }
                }
            }
            "cid" => { 
                result |= 0b10000000; 
            }
            _ => {}
        }
    }

    if result & 0b01111111 == 0b01111111 {
        return true;
    } else {
        return false;
    }
}
