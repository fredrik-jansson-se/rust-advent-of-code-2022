use std::fs;

pub fn run() -> anyhow::Result<()> {
    let input = fs::read_to_string("day15.txt").unwrap();
    println!("day15-1: {}", run_1(&input, 200000)?);
    println!("day15-2: {}", run_2(&input)?);
    Ok(())
}

fn run_1(input: &str, check_row: isize) -> anyhow::Result<usize> {
    let (_, input) = parse(input).map_err(|e| anyhow::anyhow!("{e}"))?;
    dbg! {input};
    Ok(0)
}

fn run_2(input: &str) -> anyhow::Result<usize> {
    let (_, _input) = parse(input).map_err(|e| anyhow::anyhow!("{e}"))?;
    Ok(0)
}

type Coord = (isize, isize);
type Reading = (Coord, Coord);

fn parse(i: crate::Input) -> crate::PResult<Vec<Reading>> {
    let re = regex::Regex::new(
        r#"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)"#,
    )
    .unwrap();
    let (i, captures) = nom_regex::str::re_captures(re)(i)?;

    let mut res = Vec::with_capacity(captures.len());

    for c in captures {
        let mut ints = c.iter().skip(1).map(|v| v.parse().unwrap());

        let r = (
            (ints.next().unwrap(), ints.next().unwrap()),
            (ints.next().unwrap(), ints.next().unwrap()),
        );

        res.push(r);
    }

    Ok((i, res))
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn aoc15_parse() {
        let (_, readings) = super::parse(INPUT).unwrap();
        assert_eq!(readings.len(), 14);
    }
    #[test]
    fn aoc15_run_1() {
        assert_eq!(super::run_1(INPUT, 10).unwrap(), 26);
    }

    #[test]
    fn aoc15_run_2() {
        assert_eq!(super::run_2(INPUT).unwrap(), 315);
    }
}
