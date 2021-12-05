use crate::read_lines;

pub fn part1() -> i64 {
    let lines = read_lines("./data/day18.txt");
    lines.into_iter().map(|line| calc_expr_lazy(line)).sum()
}

fn calc_expr_lazy(line: String) -> i64 {
    let mut stack_op = vec!['('];
    let mut stack_num = vec![];
    let mut s = String::new();
    for c in line.chars() {
        match c {
            '(' => {
                stack_op.push('(');
            }
            '0'..='9' => {
                s.push(c);
            }
            _ => {
                if s != "" {
                    push_num_lazy(&mut stack_num, &mut stack_op, s.parse().unwrap());
                    s = String::new();
                }
                match c {
                    ')' => {
                        stack_op.pop();
                        let x = stack_num.pop().unwrap();
                        push_num_lazy(&mut stack_num, &mut stack_op, x);
                    }
                    '+' | '*' => stack_op.push(c),
                    _ => (), // space
                }
            }
        }
    }
    if s != "" {
        push_num_lazy(&mut stack_num, &mut stack_op, s.parse().unwrap());
    }
    stack_num[0]
}

fn push_num_lazy(stack_num: &mut Vec<i64>, stack_op: &mut Vec<char>, x: i64) {
    match stack_op[stack_op.len() - 1] {
        '(' => {
            stack_num.push(x);
        }
        '+' => {
            let x = stack_num.pop().unwrap() + x;
            stack_num.push(x);
            stack_op.pop();
        }
        '*' => {
            let x = stack_num.pop().unwrap() * x;
            stack_num.push(x);
            stack_op.pop();
        }
        _ => unreachable!(),
    }
}

#[test]
fn test_18_1() {
    assert_eq!(26, calc_expr_lazy("2 * 3 + (4 * 5)".to_owned()));
    assert_eq!(
        437,
        calc_expr_lazy("5 + (8 * 3 + 9 + 3 * 4 * 3)".to_owned())
    );
    assert_eq!(
        12240,
        calc_expr_lazy("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_owned())
    );
    assert_eq!(
        13632,
        calc_expr_lazy("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_owned())
    );
}

pub fn part2() -> i64 {
    let lines = read_lines("./data/day18.txt");
    lines
        .into_iter()
        .map(|line| calc_expr_different(line))
        .sum()
}

#[inline]
fn get_precedence(c: char) -> u8 {
    match c {
        '+' => 2,
        '*' => 1,
        '(' => 0,
        _ => unreachable!(),
    }
}

fn calc_expr_different(line: String) -> i64 {
    let mut stack_op = vec!['('];
    let mut stack_num = vec![];
    let mut s = String::new();
    for c in line.chars() {
        match c {
            '(' => {
                stack_op.push('(');
            }
            '0'..='9' => {
                s.push(c);
            }
            _ => {
                if s != "" {
                    stack_num.push(s.parse().unwrap());
                    s = String::new();
                }
                match c {
                    ')' => {
                        while stack_op.last().unwrap() != &'(' {
                            calc_binary(&mut stack_num, &mut stack_op);
                        }
                        stack_op.pop();
                    }
                    '+' | '*' => {
                        while get_precedence(c) <= get_precedence(*stack_op.last().unwrap()) {
                            calc_binary(&mut stack_num, &mut stack_op);
                        }
                        stack_op.push(c)
                    }
                    _ => (), // space
                }
            }
        }
    }
    if s != "" {
        stack_num.push(s.parse().unwrap());
    }
    for _ in 1..stack_op.len() {
        calc_binary(&mut stack_num, &mut stack_op);
    }
    stack_num[0]
}

fn calc_binary(stack_num: &mut Vec<i64>, stack_op: &mut Vec<char>) {
    // dbg!(&stack_num);
    // dbg!(&stack_op);
    match stack_op.pop().unwrap() {
        '+' => {
            let x2 = stack_num.pop().unwrap();
            let x1 = stack_num.pop().unwrap();
            stack_num.push(x1 + x2);
        }
        '*' => {
            let x2 = stack_num.pop().unwrap();
            let x1 = stack_num.pop().unwrap();
            stack_num.push(x1 * x2);
        }
        _ => unreachable!(),
    }
}

#[test]
fn test_18_2() {
    assert_eq!(
        51,
        calc_expr_different("1 + (2 * 3) + (4 * (5 + 6))".to_owned())
    );
    assert_eq!(46, calc_expr_different("2 * 3 + (4 * 5)".to_owned()));
    assert_eq!(
        1445,
        calc_expr_different("5 + (8 * 3 + 9 + 3 * 4 * 3)".to_owned())
    );
    assert_eq!(
        669060,
        calc_expr_different("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_owned())
    );
    assert_eq!(
        23340,
        calc_expr_different("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_owned())
    );
}
