use std::fs;

pub fn run() -> anyhow::Result<()> {
    let input = fs::read_to_string("day6.txt").unwrap();
    println!("6:1: {}", run_1(&input)?);
    println!("6:2: {}", run_2(&input)?);
    Ok(())
}

fn simulate(input: &str, days: usize) -> anyhow::Result<usize> {
    let mut squids = parse(input)?;
    for _ in 0..days {
        let mut new_squids = vec![0; 9];
        for (idx, count) in squids.iter().enumerate().filter(|(_, count)| **count > 0) {
            if idx == 0 {
                new_squids[6] += count;
                new_squids[8] += count;
            } else {
                new_squids[idx - 1] += count;
            }
        }
        squids = new_squids;
    }
    Ok(squids.iter().sum())
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    simulate(input, 80)
}

fn run_2(input: &str) -> anyhow::Result<usize> {
    simulate(input, 256)
}

fn parse(i: &str) -> anyhow::Result<Vec<usize>> {
    let (_, input): (&str, Vec<usize>) =
        nom::multi::separated_list1(nom::bytes::complete::tag(","), crate::helper::uval)(i)
            .map_err(|e| e.to_owned())?;

    let mut res = vec![0; 9];
    for i in input {
        res[i] += 1;
    }
    Ok(res)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "3,4,3,1,2";

    #[test]
    fn aoc6_parse() {
        let squids = super::parse(INPUT).unwrap();
        assert_eq!(squids[1], 1);
        assert_eq!(squids[3], 2);
    }

    #[test]
    fn aoc6_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 5934);
    }

    #[test]
    fn aoc6_run_2() {
        assert_eq!(super::run_2(INPUT).unwrap(), 26984457539);
    }
}
