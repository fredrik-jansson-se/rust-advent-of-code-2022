pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day1.txt")?;

    println!("1:1 - {}", run_1(&input)?);
    println!("3:2 - {}", run_2(&input)?);

    Ok(())
}

use crate::{Input, PResult};

fn parse(input: Input) -> PResult<Vec<Vec<u32>>> {
    let (i, res) = nom::multi::separated_list1(
        nom::multi::many_m_n(2, 2, nom::character::complete::newline),
        nom::multi::separated_list1(
            nom::character::complete::newline,
            nom::character::complete::u32,
        ),
    )(input)?;
    Ok((i, res))
}

fn run_1(input_txt: &str) -> anyhow::Result<u32> {
    let (_, input) = parse(input_txt).map_err(|e| anyhow::anyhow!("{e}"))?;
    let max = input.iter().map(|elf| elf.iter().sum()).max().unwrap();
    Ok(max)
}

fn run_2(input_txt: &str) -> anyhow::Result<u32> {
    let (_, input) = parse(input_txt).map_err(|e| anyhow::anyhow!("{e}"))?;
    let mut sums: Vec<u32> = input.iter().map(|elf| elf.iter().sum()).collect();
    sums.sort_by(|a, b| b.cmp(a));

    let top_3 = sums.iter().take(3).sum();
    Ok(top_3)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn aoc1_parse() {
        let (_, result) = super::parse(INPUT).unwrap();
        assert_eq!(result.len(), 5);
        assert_eq!(result[0].len(), 3);
    }

    #[test]
    fn aoc1_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 24000);
    }

    #[test]
    fn aoc1_run_2() {
        assert_eq!(super::run_2(INPUT).unwrap(), 45000);
    }
}
