use crate::{Input, PResult};
use std::fs;

pub fn run() -> anyhow::Result<()> {
    let input = fs::read_to_string("day4.txt").unwrap();
    println!("4:1 {}", run_1(&input)?);
    println!("4:2 {}", run_2(&input)?);
    Ok(())
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let (_i, sections) = parse(input).map_err(|e| anyhow::anyhow!("{e}"))?;

    let mut cnt = 0;

    for section in sections {
        let sa = section.0;
        let sb = section.1;

        if (sb.start() >= sa.start() && sb.end() <= sa.end())
            || (sa.start() >= sb.start() && sa.end() <= sb.end())
        {
            cnt += 1;
        }
    }

    Ok(cnt)
}

fn run_2(input: &str) -> anyhow::Result<usize> {
    let (_i, sections) = parse(input).map_err(|e| anyhow::anyhow!("{e}"))?;

    let mut cnt = 0;

    for section in sections {
        let sa = section.0;
        let sb = section.1;

        if (sb.start() >= sa.start() && sb.start() <= sa.end())
            || (sa.start() >= sb.start() && sa.start() <= sb.end())
        {
            cnt += 1;
        }
    }

    Ok(cnt)
}
type Section = std::ops::RangeInclusive<u32>;
fn parse_section(i: Input) -> PResult<Section> {
    use nom::{bytes::complete::tag, character::complete::u32, sequence::separated_pair};

    let (i, (low, high)) = separated_pair(u32, tag("-"), u32)(i)?;

    Ok((i, std::ops::RangeInclusive::new(low, high)))
}

fn parse(i: Input) -> PResult<Vec<(Section, Section)>> {
    use nom::{
        bytes::complete::tag, character::complete::newline, multi::separated_list1,
        sequence::separated_pair,
    };

    let sections = separated_pair(parse_section, tag(","), parse_section);

    let r = separated_list1(newline, sections)(i)?;
    Ok(r)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

    #[test]
    fn aoc4_parse() {
        assert_eq!(
            super::parse_section("1-11").unwrap().1,
            std::ops::RangeInclusive::new(1, 11)
        );
        // let (_input, boards) = super::parse(INPUT).unwrap();
        // assert_eq!(boards.len(), 3);
        let res = super::parse(INPUT).unwrap().1;
        assert_eq!(res.len(), 6);
    }

    #[test]
    fn aoc4_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 2)
    }

    #[test]
    fn aoc4_run_2() {
        assert_eq!(super::run_2(INPUT).unwrap(), 4)
    }
}
