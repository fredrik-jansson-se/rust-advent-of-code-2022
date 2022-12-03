use std::fs;

pub fn run() -> anyhow::Result<()> {
    let input = fs::read_to_string("day3.txt")?;

    println!("3:1 - {}", run_1(&input)?);
    println!("3:2 - {}", run_2(&input)?);
    Ok(())
}

fn priority(c: char) -> usize {
    const A_L: usize = 'a' as usize;
    const A_U: usize = 'A' as usize;
    if c.is_lowercase() {
        c as usize - A_L + 1
    } else {
        c as usize - A_U + 27
    }
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let mut sum = 0;
    for line in input.lines() {
        let l: Vec<char> = line.chars().collect();
        let low = &l[..l.len() / 2];
        let high = &l[(l.len() / 2)..];
        // println!("{low:?} {high:?}");
        if let Some(in_both) = low.iter().find(|c| high.contains(c)) {
            sum += priority(*in_both);
        }
    }

    Ok(sum)
}

fn run_2(input: &str) -> anyhow::Result<usize> {
    use std::collections::HashSet;
    let mut sum = 0;
    let groups: Vec<&str> = input.lines().collect();

    for group3 in groups.chunks(3) {
        let mut first: HashSet<_> = group3[0].chars().collect();
        first.retain(|f| group3[1].contains(|c| c == *f) && group3[2].contains(|c| c == *f));
        sum += priority(*first.iter().next().unwrap());
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn aoc3_priority() {
        assert_eq!(super::priority('a'), 1);
        assert_eq!(super::priority('z'), 26);
        assert_eq!(super::priority('A'), 27);
        assert_eq!(super::priority('Z'), 52);
    }
    #[test]
    fn aoc3_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 157);
    }

    #[test]
    fn aoc3_run_2() {
        assert_eq!(super::run_2(INPUT).unwrap(), 70);
    }
}
