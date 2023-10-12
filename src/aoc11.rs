use std::fs;

use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, anychar, newline, space0, space1, u64},
    combinator::opt,
    multi::separated_list0,
    sequence::{delimited, terminated},
};

pub fn run() -> anyhow::Result<()> {
    let input = fs::read_to_string("day11.txt").unwrap();
    println!("day11-1: {}", run_1(&input)?);
    println!("day11-2: {}", run_2(&input)?);
    Ok(())
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let (_, mut monkeys) = parse(input).map_err(|e| anyhow::anyhow!(e.to_string()))?;

    let mut inspection_count = vec![0; monkeys.len()];

    for _round in 0..20 {
        for m in 0..monkeys.len() {
            monkeys[m].items.reverse();
            while let Some(item) = monkeys[m].items.pop() {
                inspection_count[m] += 1;
                let item = monkeys[m].operation.calc(item);
                let item = item / 3;

                if item % monkeys[m].test_div_by == 0 {
                    let idx = monkeys[m].true_to as usize;
                    monkeys[idx].items.push(item);
                } else {
                    let idx = monkeys[m].false_to as usize;
                    monkeys[idx].items.push(item);
                }
            }
        }
    }

    inspection_count.sort();
    Ok(inspection_count[inspection_count.len() - 2] * inspection_count[inspection_count.len() - 1])
}

fn run_2(input: &str) -> anyhow::Result<usize> {
    let (_, mut monkeys) = parse(input).map_err(|e| anyhow::anyhow!(e.to_string()))?;

    let modulo = monkeys.iter().fold(1, |m, monkey| m * monkey.test_div_by);

    let mut inspection_count = vec![0; monkeys.len()];

    for _round in 0..10_000 {
        for m in 0..monkeys.len() {
            monkeys[m].items.reverse();
            while let Some(item) = monkeys[m].items.pop() {
                inspection_count[m] += 1;
                let item = monkeys[m].operation.calc(item);
                let item = item % modulo;

                if item % monkeys[m].test_div_by == 0 {
                    let idx = monkeys[m].true_to as usize;
                    monkeys[idx].items.push(item);
                } else {
                    let idx = monkeys[m].false_to as usize;
                    monkeys[idx].items.push(item);
                }
            }
        }
    }

    inspection_count.sort();
    Ok(inspection_count[inspection_count.len() - 2] * inspection_count[inspection_count.len() - 1])
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test_div_by: u64,
    true_to: u64,
    false_to: u64,
}

#[derive(Debug)]
struct Operation {
    a: String,
    b: String,
    op: char,
}

impl Operation {
    fn calc(&self, old: u64) -> u64 {
        let a = if self.a == "old" {
            old
        } else {
            self.a.parse().unwrap()
        };
        let b = if self.b == "old" {
            old
        } else {
            self.b.parse().unwrap()
        };

        match self.op {
            '+' => a + b,
            '-' => a - b,
            '*' => a * b,
            '/' => a / b,
            _ => unreachable!(),
        }
    }
}

fn parse_operation(i: crate::Input) -> crate::PResult<Operation> {
    let (i, _) = tag("new = ")(i)?;
    let (i, a) = terminated(alphanumeric1, space1)(i)?;
    let (i, op) = terminated(anychar, space1)(i)?;
    let (i, b) = terminated(alphanumeric1, space0)(i)?;

    Ok((
        i,
        Operation {
            a: a.to_string(),
            op,
            b: b.to_string(),
        },
    ))
}

fn parse_monkey(i: crate::Input) -> crate::PResult<Monkey> {
    let (i, _name) = delimited(tag("Monkey "), u64, tag(":\n"))(i)?;
    let (i, items) = delimited(
        tag("  Starting items: "),
        separated_list0(tag(", "), u64),
        newline,
    )(i)?;

    let (i, operation) = delimited(tag("  Operation: "), parse_operation, newline)(i)?;

    let (i, test_div_by) = delimited(tag("  Test: divisible by "), u64, newline)(i)?;

    let (i, true_to) = delimited(tag("    If true: throw to monkey "), u64, newline)(i)?;
    let (i, false_to) = delimited(tag("    If false: throw to monkey "), u64, opt(newline))(i)?;

    Ok((
        i,
        Monkey {
            items,
            operation,
            test_div_by,
            true_to,
            false_to,
        },
    ))
}

fn parse(i: crate::Input) -> crate::PResult<Vec<Monkey>> {
    let res = separated_list0(newline, parse_monkey)(i)?;
    Ok(res)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn aoc11_parse() {
        let (_, monkeys) = super::parse(INPUT).unwrap();
        assert_eq!(monkeys.len(), 4);
    }

    #[test]
    fn aoc11_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 10605);
    }

    #[test]
    fn aoc11_run_2() {
        assert_eq!(super::run_2(INPUT).unwrap(), 2713310158);
    }
}
