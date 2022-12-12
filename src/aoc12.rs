use std::{collections::HashMap, fs};

pub fn run() -> anyhow::Result<()> {
    let input = fs::read_to_string("day12.txt").unwrap();
    println!("day12-1: {}", run_1(&input)?);
    println!("day12-2: {}", run_2(&input)?);
    Ok(())
}

type Coord = (isize, isize);
type Map = HashMap<Coord, usize>;

fn get_nbrs(current: Coord, map: &Map) -> Vec<(Coord, usize)> {
    let (x, y) = current;
    let mut nbrs = vec![
        ((x - 1, y), 1),
        ((x + 1, y), 1),
        ((x, y - 1), 1),
        ((x, y + 1), 1),
    ];

    let cur_height = *map.get(&current).unwrap();

    nbrs.retain(|(p, _)| cur_height >= (*map.get(p).unwrap_or(&usize::MAX) - 1));
    nbrs
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let (map, start, end) = parse(input);

    let path = pathfinding::directed::astar::astar(
        &start,
        |c| get_nbrs(*c, &map),
        |c| ((c.0 - end.0) * (c.0 - end.0) + (c.1 - end.1) * (c.1 - end.1)) as usize,
        |c| *c == end,
    )
    .unwrap();
    Ok(path.0.len() - 1)
}

fn run_2(input: &str) -> anyhow::Result<usize> {
    let (map, _start, end) = parse(input);

    let mut min = usize::MAX;
    for start in map.iter().filter(|(_, h)| **h == ('a' as usize)) {
        let (start, _) = start;
        if let Some(path) = pathfinding::directed::astar::astar(
            start,
            |c| get_nbrs(*c, &map),
            |c| ((c.0 - end.0) * (c.0 - end.0) + (c.1 - end.1) * (c.1 - end.1)) as usize,
            |c| *c == end,
        ) {
            min = min.min(path.0.len() - 1);
        }
    }
    Ok(min)
}

fn parse(input: &str) -> (Map, Coord, Coord) {
    let mut map: HashMap<(isize, isize), usize> = HashMap::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            let c = match c {
                'S' => {
                    start = (row as isize, col as isize);
                    'a'
                }
                'E' => {
                    end = (row as isize, col as isize);
                    'z'
                }
                c => c,
            };
            map.insert((row as isize, col as isize), c as usize);
        }
    }

    (map, start, end)
}

#[cfg(test)]
mod tests {
    const INPUT_1: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn aoc12_run_1() {
        assert_eq!(super::run_1(INPUT_1).unwrap(), 31);
    }

    #[test]
    fn aoc12_run_2() {
        assert_eq!(super::run_2(INPUT_1).unwrap(), 29);
    }
}
