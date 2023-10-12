use std::fs;

pub fn run() -> anyhow::Result<()> {
    let input = fs::read_to_string("day14.txt").unwrap();
    println!("day14-1: {}", run_1(&input)?);
    println!("day14-2: {}", run_2(&input)?);
    Ok(())
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let (_, paths) = parse(input).map_err(|e| anyhow::anyhow!(e.to_string()))?;

    const SOURCE: Coord = Coord(500, 0);

    let mut map: Map = Default::default();
    let mut max_y = i64::MIN;
    for path in paths {
        for se in path.windows(2) {
            let start = &se[0];
            let end = &se[1];
            max_y = max_y.max(start.1.max(end.1));

            let dx = end.0 - start.0;
            let dy = end.1 - start.1;
            let path_len = dx.abs() + dy.abs();
            let dx = dx.clamp(-1, 1);
            let dy = dy.clamp(-1, 1);

            for i in 0..=path_len {
                map.insert(Coord(start.0 + i * dx, start.1 + i * dy));
            }
        }
    }

    let mut cnt = 0;
    loop {
        let mut sand = SOURCE;
        loop {
            let down = Coord(sand.0, sand.1 + 1);
            let down_left = Coord(sand.0 - 1, sand.1 + 1);
            let down_right = Coord(sand.0 + 1, sand.1 + 1);

            if !map.contains(&down) {
                sand = down;
                if sand.1 > max_y {
                    break;
                }
            } else if !map.contains(&down_left) {
                sand = down_left;
            } else if !map.contains(&down_right) {
                sand = down_right;
            } else {
                break;
            }
        }
        map.insert(sand);
        if sand.1 > max_y {
            break;
        }
        cnt += 1;
    }
    Ok(cnt)
}

fn run_2(input: &str) -> anyhow::Result<usize> {
    let (_, paths) = parse(input).map_err(|e| anyhow::anyhow!(e.to_string()))?;

    const SOURCE: Coord = Coord(500, 0);

    let mut map: Map = Default::default();
    let mut max_y = i64::MIN;
    for path in paths {
        for se in path.windows(2) {
            let start = &se[0];
            let end = &se[1];
            max_y = max_y.max(start.1.max(end.1));

            let dx = end.0 - start.0;
            let dy = end.1 - start.1;
            let path_len = dx.abs() + dy.abs();
            let dx = dx.clamp(-1, 1);
            let dy = dy.clamp(-1, 1);

            for i in 0..=path_len {
                map.insert(Coord(start.0 + i * dx, start.1 + i * dy));
            }
        }
    }

    let mut cnt = 0;
    let floor_y = max_y + 2;
    loop {
        let mut sand = SOURCE;
        loop {
            let down = Coord(sand.0, sand.1 + 1);

            // Always stop ad the floor
            if down.1 == floor_y {
                break;
            }
            let down_left = Coord(sand.0 - 1, sand.1 + 1);
            let down_right = Coord(sand.0 + 1, sand.1 + 1);

            if !map.contains(&down) {
                sand = down;
            } else if !map.contains(&down_left) {
                sand = down_left;
            } else if !map.contains(&down_right) {
                sand = down_right;
            } else {
                break;
            }

        }
        map.insert(sand);
        cnt += 1;
        if sand == SOURCE {
            break;
        }
    }
    Ok(cnt)
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Coord(i64, i64);

type Map = std::collections::HashSet<Coord>;

fn parse(i: crate::Input) -> crate::PResult<Vec<Vec<Coord>>> {
    let coord = nom::combinator::map(
        nom::sequence::separated_pair(
            nom::character::complete::i64,
            nom::bytes::complete::tag(","),
            nom::character::complete::i64,
        ),
        |(x, y)| Coord(x, y),
    );
    let path = nom::multi::separated_list1(nom::bytes::complete::tag(" -> "), coord);

    let mut paths = nom::multi::separated_list1(nom::character::complete::newline, path);

    let res = paths(i)?;
    Ok(res)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
    #[test]
    fn aoc14_parse() {
        let (_, paths) = super::parse(INPUT).unwrap();
        assert_eq!(paths.len(), 2);
        assert_eq!(paths[0].len(), 3);
        assert_eq!(paths[1].len(), 4);
    }
    #[test]
    fn aoc14_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 24);
    }
    #[test]
    fn aoc14_run_2() {
        assert_eq!(super::run_2(INPUT).unwrap(), 93);
    }
}
