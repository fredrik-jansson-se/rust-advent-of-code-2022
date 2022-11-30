use std::fs;

pub fn run() -> anyhow::Result<()> {
    let input = fs::read_to_string("day7.txt")?;
    println!("day7-1: {}", run_1(&input)?);
    println!("day7-2: {}", run_2(&input)?);
    Ok(())
}

pub fn run_1(input: &str) -> anyhow::Result<isize> {
    let mut positions = parse(input)?;
    positions.sort_unstable();

    let median_pos: isize = positions[positions.len() / 2];

    let fuel: isize = positions.iter().map(|p| (p - median_pos).abs()).sum();

    Ok(fuel)
}

fn fuel_2(from: isize, to: isize) -> isize {
    let low = to.min(from);
    let high = to.max(from) - low;

    (high + 1) * high / 2
}

pub fn run_2(input: &str) -> anyhow::Result<isize> {
    let mut positions = parse(input)?;
    positions.sort_unstable();

    let low = positions[0];
    let high = positions[positions.len() - 1];
    let mut min_fuel = isize::MAX;
    for to in low..=high {
        let om = min_fuel;
        min_fuel = min_fuel.min(positions.iter().map(|from| fuel_2(*from, to)).sum());
        if min_fuel < om {
            // dbg! {(positions[idx], min_fuel)};
        }
    }
    Ok(min_fuel)
}

// 100757428 too high

fn parse(i: &str) -> anyhow::Result<Vec<isize>> {
    let (_, res) =
        nom::multi::separated_list1(nom::bytes::complete::tag(","), crate::helper::ival)(i)
            .map_err(|v| v.to_owned())?;
    Ok(res)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn aoc7_parse() {
        let pos = super::parse(INPUT).unwrap();
        assert_eq!(pos.len(), 10);
    }

    #[test]
    fn aoc7_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 37);
    }

    #[test]
    fn aoc7_run_2() {
        assert_eq!(super::fuel_2(16, 5), 66);
        assert_eq!(super::fuel_2(1, 5), 10);
        assert_eq!(super::fuel_2(2, 5), 6);
        assert_eq!(super::fuel_2(14, 5), 45);
        assert_eq!(super::run_2(INPUT).unwrap(), 168);
    }
}
