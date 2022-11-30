use std::fs;

pub fn run() -> anyhow::Result<()> {
    let input = fs::read_to_string("day10.txt").unwrap();
    println!("day10-1: {}", run_1(&input)?);
    println!("day10-2: {}", run_2(&input)?);
    Ok(())
}

enum Res {
    Ok,
    Invalid(char),
    Incomplete(Vec<char>),
}

fn check(i: &str) -> Res {
    let mut stack = Vec::new();

    for c in i.chars() {
        match c {
            '[' | '(' | '{' | '<' => {
                stack.push(c);
            }
            ']' => {
                let c2 = stack.pop().unwrap();
                if c2 != '[' {
                    return Res::Invalid(c);
                }
            }
            ')' => {
                let c2 = stack.pop().unwrap();
                if c2 != '(' {
                    return Res::Invalid(c);
                }
            }
            '}' => {
                let c2 = stack.pop().unwrap();
                if c2 != '{' {
                    return Res::Invalid(c);
                }
            }
            '>' => {
                let c2 = stack.pop().unwrap();
                if c2 != '<' {
                    return Res::Invalid(c);
                }
            }
            _ => unreachable!(),
        }
    }

    if stack.is_empty() {
        Res::Ok
    } else {
        Res::Incomplete(stack)
    }
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let invalids = input
        .lines()
        .map(check)
        .filter_map(|r| match r {
            Res::Invalid(c) => Some(c),
            _ => None,
        })
        .map(|c| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => unreachable!(),
        })
        .sum();
    Ok(invalids)
}

fn complete(mut stack: Vec<char>) -> usize {
    let mut scores = Vec::new();

    while let Some(c) = stack.pop() {
        match c {
            '(' => scores.push(1),
            '[' => scores.push(2),
            '{' => scores.push(3),
            '<' => scores.push(4),
            _ => unreachable!(),
        }
    }

    scores.iter().fold(0, |p, s| p * 5 + s)
}

fn run_2(input: &str) -> anyhow::Result<usize> {
    let mut invalids = input
        .lines()
        .map(check)
        .filter_map(|r| match r {
            Res::Incomplete(c) => Some(c),
            _ => None,
        })
        .map(complete)
        .collect::<Vec<_>>();

    invalids.sort_unstable();

    Ok(invalids[invalids.len() / 2])
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn aoc10_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 26397);
    }

    #[test]
    fn aoc10_run_2() {
        assert_eq!(super::run_2(INPUT).unwrap(), 288957);
    }
}
