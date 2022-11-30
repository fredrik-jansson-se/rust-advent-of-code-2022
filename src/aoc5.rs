use std::fs;

pub fn run() -> anyhow::Result<()> {
    let input = fs::read_to_string("day5.txt").unwrap();

    println!("5:1 {}", run_1(&input)?);
    println!("5:2 {}", run_2(&input)?);
    Ok(())
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let lines: Vec<_> = parse(input)?;

    let mut coord_cnt = std::collections::HashMap::new();

    for (c1, c2) in lines.iter() {
        // vertical
        if c1.0 == c2.0 {
            // println!("{},{} -> {},{}", c1.0, c1.1, c2.0, c2.1);
            let x = c1.0;
            for y in c1.1.min(c2.1)..=c1.1.max(c2.1) {
                // println!("\t{},{}", x, y);
                *coord_cnt.entry((x, y)).or_insert(0) += 1;
            }
        }
        // horizontal
        else if c1.1 == c2.1 {
            let y = c1.1;
            for x in c1.0.min(c2.0)..=c1.0.max(c2.0) {
                *coord_cnt.entry((x, y)).or_insert(0) += 1;
            }
        }
    }

    Ok(coord_cnt.iter().filter(|(_, v)| **v >= 2).count())
}

fn run_2(input: &str) -> anyhow::Result<usize> {
    let lines: Vec<_> = parse(input)?;

    let mut coord_cnt = std::collections::HashMap::new();

    for (c1, c2) in lines.iter() {
        // vertical
        if c1.0 == c2.0 {
            // println!("{},{} -> {},{}", c1.0, c1.1, c2.0, c2.1);
            let x = c1.0;
            for y in c1.1.min(c2.1)..=c1.1.max(c2.1) {
                // println!("\t{},{}", x, y);
                *coord_cnt.entry((x, y)).or_insert(0) += 1;
            }
        } else {
            let (c1, c2) = if c1.0 < c2.0 { (c1, c2) } else { (c2, c1) };
            let k = (c2.1 - c1.1) / (c2.0 - c1.0);
            for x in c1.0..=c2.0 {
                let y = c1.1 + k * (x - c1.0);
                *coord_cnt.entry((x, y)).or_insert(0) += 1;
            }
        }
    }

    Ok(coord_cnt.iter().filter(|(_, v)| **v >= 2).count())
}

type Coord = (isize, isize);

fn parse(i: &str) -> anyhow::Result<Vec<(Coord, Coord)>> {
    let coord = |i| {
        nom::sequence::separated_pair(
            crate::helper::ival,
            nom::bytes::complete::tag(","),
            crate::helper::ival,
        )(i)
    };

    let line = nom::sequence::separated_pair(coord, nom::bytes::complete::tag(" -> "), coord);

    let (_, lines) = nom::multi::separated_list1(nom::character::complete::newline, line)(i)
        .map_err(|e| e.to_owned())?;

    Ok(lines)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn aoc5_parse() {
        let lines = super::parse(INPUT).unwrap();
        assert_eq!(lines.len(), 10);
        assert_eq!(lines[0].0, (0, 9));
        assert_eq!(lines[9].1, (8, 2));
    }

    #[test]
    fn aoc5_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 5);
    }
    #[test]
    fn aoc5_run_2() {
        assert_eq!(super::run_2(INPUT).unwrap(), 12);
    }
}
