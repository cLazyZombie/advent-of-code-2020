fn main() {
    let input = include_str!("input.txt");
    let answers = solve(input);
    println!("part1: {}", answers.0);
    println!("part2: {}", answers.1);
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Op {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

fn solve(input: &str) -> (i32, i32) {
    let mut ops = Vec::new();
    for line in input.lines() {
        ops.push(parse_op(line));
    }

    let acc = run(&ops);
    let acc_2 = fix_ops(&ops);

    (acc.1, acc_2)
}

fn fix_ops(ops: &Vec<Op>) -> i32 {
    let mut ops = ops.clone();

    for i in 0..ops.len() {
        match ops[i] {
            Op::Nop(v) => {
                // change nop to jmp
                ops[i] = Op::Jmp(v);
                let result = run(&ops);
                if result.0 {
                    return result.1;
                }

                // revert
                ops[i] = Op::Nop(v);
            }
            Op::Jmp(v) => {
                // change jmp to nop
                ops[i] = Op::Nop(v);
                let result = run(&ops);
                if result.0 {
                    return result.1;
                }

                // revert
                ops[i] = Op::Jmp(v);
            }
            _ => {}
        }
    }

    return -1;
}

fn parse_op(line: &str) -> Op {
    let toks = line.split_whitespace().collect::<Vec<_>>();

    let arg = toks[1].parse::<i32>().unwrap();

    match toks[0] {
        "nop" => {
            return Op::Nop(arg);
        }
        "acc" => {
            return Op::Acc(arg);
        }
        "jmp" => {
            return Op::Jmp(arg);
        }
        _ => {
            panic!("invalid tokens {}", toks[0]);
        }
    }
}

// run program and get result
// result.0 : teminated normally (true) or stop before infinity loop(false)
// result.1 : acc value
fn run(ops: &Vec<Op>) -> (bool, i32) {
    let mut executed = vec![false; ops.len()];
    let mut acc = 0;

    let mut pc = 0;
    loop {
        if pc == ops.len() {
            return (true, acc);
        }

        if executed[pc] {
            return (false, acc);
        }

        executed[pc] = true;

        match ops[pc] {
            Op::Nop(_) => {
                pc += 1;
            }
            Op::Acc(v) => {
                pc += 1;
                acc += v;
            }
            Op::Jmp(v) => {
                pc = (pc as i32 + v) as usize;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_op() {
        assert_eq!(parse_op("nop +0"), Op::Nop(0));
        assert_eq!(parse_op("jmp +8"), Op::Jmp(8));
        assert_eq!(parse_op("acc -1"), Op::Acc(-1));
    }

    #[test]
    fn test_run() {
        let ops = vec![
            Op::Nop(0),
            Op::Acc(1),
            Op::Jmp(4),
            Op::Acc(3),
            Op::Jmp(-3),
            Op::Acc(-99),
            Op::Acc(1),
            Op::Jmp(-4),
            Op::Acc(6),
        ];

        let acc = run(&ops);
        assert_eq!(acc, (false, 5));
    }

    #[test]
    fn test_fix_ops() {
        let ops = vec![
            Op::Nop(0),
            Op::Acc(1),
            Op::Jmp(4),
            Op::Acc(3),
            Op::Jmp(-3),
            Op::Acc(-99),
            Op::Acc(1),
            Op::Jmp(-4),
            Op::Acc(6),
        ];

        let acc = fix_ops(&ops);
        assert_eq!(acc, 8);
    }
}
