use std::fs;

type Coord = (isize, isize);

type Map = std::collections::HashMap<Coord, bool>;
type Alg = Vec<bool>;

pub fn run() -> anyhow::Result<()> {
    let input = fs::read_to_string("day20.txt")?;
    println!("20:1: {}", run_1(&input, false)?);
    println!("20:2: {}", run_2(&input, false)?);
    Ok(())
}

type Input<'a> = &'a str;
type PResult<'a, O> = nom::IResult<Input<'a>, O, nom::error::VerboseError<Input<'a>>>;

fn parse_alg(i: Input) -> PResult<Alg> {
    use nom::{branch::alt, bytes::complete::tag, combinator::map, multi::many_m_n};
    let light = map(tag("#"), |_| true);
    let dark = map(tag("."), |_| false);

    let (i, alg) = many_m_n(512, 512, alt((light, dark)))(i)?;
    let (i, _) = nom::combinator::opt(nom::character::complete::newline)(i)?;
    Ok((i, alg))
}

fn parse_map(i: Input) -> PResult<Map> {
    let mut map = Map::new();

    for (r_num, row) in i.lines().enumerate() {
        for (c_num, c) in row.chars().enumerate() {
            map.insert((r_num as isize, c_num as isize), c == '#');
        }
    }

    Ok(("", map))
}

fn parse(i: Input) -> PResult<(Alg, Map)> {
    let (i, alg) = parse_alg(i)?;
    let (i, _) = nom::character::complete::newline(i)?;
    let (i, map) = parse_map(i)?;
    Ok((i, (alg, map)))
}

fn get_num((rc, cc): Coord, map: &Map, default: bool) -> usize {
    let mut num = 0;

    for r in (rc - 1)..=(rc + 1) {
        for c in (cc - 1)..=(cc + 1) {
            num <<= 1;
            if *map.get(&(r, c)).unwrap_or(&default) {
                num += 1;
            }
        }
    }

    num
}

fn enhance(input: &str, test: bool, rounds: usize) -> anyhow::Result<usize> {
    let (_, (alg, mut map)) = parse(input).unwrap();

    let mut new_map = Map::new();

    for round in 0..rounds {
        let default = !test && (round % 2 == 1);

        let (min_row, max_row, min_col, max_col) = map.keys().fold(
            (isize::MAX, isize::MIN, isize::MAX, isize::MIN),
            |(rmin, rmax, cmin, cmax), (r, c)| {
                (rmin.min(*r), rmax.max(*r), cmin.min(*c), cmax.max(*c))
            },
        );

        for r in (min_row - 1)..=(max_row + 1) {
            for c in (min_col - 1)..=(max_col + 1) {
                new_map.insert((r, c), alg[get_num((r, c), &map, default)]);
            }
        }
        std::mem::swap(&mut map, &mut new_map);
        new_map.clear();
    }
    let cnt = map.iter().filter(|(_, v)| **v).count();
    Ok(cnt)
}

fn run_1(input: &str, test: bool) -> anyhow::Result<usize> {
    enhance(input, test, 2)
}

fn run_2(input: &str, test: bool) -> anyhow::Result<usize> {
    enhance(input, test, 50)
}

#[cfg(test)]
mod tests {
    const ALG:&str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#";
    const MAP: &str = "#..#.
#....
##..#
..#..
..###";

    #[test]
    fn aoc20_parse() {
        let (_, alg_1) = super::parse_alg(ALG).unwrap();
        let (_, map_1) = super::parse_map(MAP).unwrap();
        // assert_eq!(map_1.len(), 10);

        let (_, (alg_2, map_2)) = super::parse(&format!("{ALG}\n\n{MAP}")).unwrap();
        assert_eq!(map_1, map_2);
        assert_eq!(alg_1, alg_2);
    }

    #[test]
    fn aoc20_get_num() {
        let i = "...
#..
.#.";
        let (_, map) = super::parse_map(i).unwrap();

        assert_eq!(super::get_num((1, 1), &map, false), 34);
    }

    #[test]
    fn aoc20_run_1() {
        let input = format!("{ALG}\n\n{MAP}");
        assert_eq!(super::run_1(&input, true).unwrap(), 35);
    }

    #[test]
    fn aoc20_run_2() {
        let input = format!("{ALG}\n\n{MAP}");
        assert_eq!(super::run_2(&input, true).unwrap(), 3351);
    }
}
