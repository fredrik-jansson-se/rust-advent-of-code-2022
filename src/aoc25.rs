use std::collections::HashMap;
use std::fs;

pub fn run() -> anyhow::Result<()> {
    let input = fs::read_to_string("day25.txt").unwrap();
    println!("day25-1: {}", run_1(&input)?);
    println!("day25-2: {}", run_2(&input)?);
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Dir {
    East,
    South,
}

// fn next_step(width: usize, height: usize) -> impl Fn(Dir, (usize, usize)) -> (usize, usize) {
//     move |dir, (row, col)| match dir {
//         Dir::East => (row, (col + 1) % width),
//         Dir::South => ((row + 1) % height, col),
//     }
// }

fn run_1(input: &str) -> anyhow::Result<usize> {
    let (_, (width, height, mut map)) = parse(input).map_err(|e| e.to_owned())?;
    let mut cnt = 0;

    loop {
        let mut cur = HashMap::new();
        cnt += 1;

        for ((row, col), _) in map.iter().filter(|((_, _), dir)| **dir == Dir::East) {
            let next_col = (col + 1) % width;
            if map.get(&(*row, next_col)).is_none() {
                cur.insert((*row, next_col), Dir::East);
            } else {
                cur.insert((*row, *col), Dir::East);
            }
        }

        for ((row, col), _) in map.iter().filter(|((_, _), dir)| **dir == Dir::South) {
            let next_row = (row + 1) % height;
            let occupied = map.get(&(next_row, *col));
            let occupied = match occupied {
                Some(Dir::South) => true,
                _ => cur.get(&(next_row, *col)).is_some(),
            };
            if !occupied {
                cur.insert((next_row, *col), Dir::South);
            } else {
                cur.insert((*row, *col), Dir::South);
            }
        }

        if map == cur {
            break;
        }

        map = cur;
    }

    Ok(cnt)
}

fn run_2(_input: &str) -> anyhow::Result<usize> {
    todo!()
}

// fn parse_pos(i: &str) -> nom::IResult<&str, Option<Dir>> {
//     let east = nom::combinator::map(nom::bytes::complete::tag(">"), |_| Some(Dir::East));
//     let south = nom::combinator::map(nom::bytes::complete::tag("v"), |_| Some(Dir::South));
//     let empty = nom::combinator::map(nom::bytes::complete::tag("."), |_| None);
//     nom::branch::alt((east, south, empty))(i)
// }

type Map = HashMap<(usize, usize), Dir>;
fn parse(i: &str) -> nom::IResult<&str, (usize, usize, Map)> {
    let mut res = HashMap::new();
    let mut width = 0;
    let mut height = 0;
    for (row, line) in i.lines().enumerate() {
        height = height.max(row);
        for (col, c) in line.chars().enumerate() {
            width = width.max(col);
            match c {
                '>' => {
                    res.insert((row, col), Dir::East);
                }
                'v' => {
                    res.insert((row, col), Dir::South);
                }
                '.' => (),
                _ => unreachable!(),
            };
        }
    }

    Ok(("", (width + 1, height + 1, res)))
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";

    #[test]
    fn aoc25_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 58);
    }
}
