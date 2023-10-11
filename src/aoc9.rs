use std::fs;

pub fn run() -> anyhow::Result<()> {
    let input = fs::read_to_string("day9.txt").unwrap();
    println!("day9-1: {}", run_1(&input)?);
    println!("day9-2: {}", run_2(&input)?);
    Ok(())
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let (_, moves) = parse(input).map_err(|e| anyhow::anyhow!(e.to_string()))?;

    let mut tail_visited = std::collections::HashSet::new();
    let mut head = Coord(0, 0);
    let mut tail = Coord(0, 0);
    tail_visited.insert(tail);
    for m in moves {
        let (num_moves, dx, dy) = match m {
            Move::Left(m) => (m, -1, 0),
            Move::Right(m) => (m, 1, 0),
            Move::Up(m) => (m, 0, 1),
            Move::Down(m) => (m, 0, -1),
        };

        for _ in 0..num_moves {
            head.0 += dx;
            head.1 += dy;

            // need to move tail
            if head.sq_distance(&tail) > 2 {
                let dx = (head.0 - tail.0).clamp(-1, 1);
                let dy = (head.1 - tail.1).clamp(-1, 1);
                tail.0 += dx;
                tail.1 += dy;
            }
            tail_visited.insert(tail);
        }

    }

    Ok(tail_visited.len())
}

fn run_2(input: &str) -> anyhow::Result<usize> {
    let (_, moves) = parse(input).map_err(|e| anyhow::anyhow!(e.to_string()))?;
    let mut rope = [Coord(0,0);10];
    let mut tail_visited = std::collections::HashSet::new();
    tail_visited.insert(Coord(0,0));
    for m in moves {
        let (num_moves, dx, dy) = match m {
            Move::Left(m) => (m, -1, 0),
            Move::Right(m) => (m, 1, 0),
            Move::Up(m) => (m, 0, 1),
            Move::Down(m) => (m, 0, -1),
        };

        for _ in 0..num_moves {
            rope[0].0 += dx;
            rope[0].1 += dy;

            for i in 1..rope.len() {
                let head = rope[i-1];
                let tail = &mut rope[i];
                if head.sq_distance(tail) > 2 {
                    let dx = (head.0 - tail.0).clamp(-1, 1);
                    let dy = (head.1 - tail.1).clamp(-1, 1);
                    tail.0 += dx;
                    tail.1 += dy;
                }
            }
            tail_visited.insert(rope[9]);
        }

    }
    Ok(tail_visited.len())
}

#[derive(Debug)]
enum Move {
    Left(isize),
    Right(isize),
    Up(isize),
    Down(isize),
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Coord(isize, isize);

impl Coord {
    fn sq_distance(&self, other: &Self) -> usize {
        ((self.0 - other.0).pow(2) + (self.1 - other.1).pow(2)) as usize
    }
}

fn parse(i: crate::Input) -> crate::PResult<Vec<Move>> {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{i64, newline},
        combinator::map,
        multi::separated_list1,
        sequence::preceded,
    };
    let up = map(preceded(tag("U "), i64), |m| Move::Up(m as isize));
    let down = map(preceded(tag("D "), i64), |m| Move::Down(m as isize));
    let left = map(preceded(tag("L "), i64), |m| Move::Left(m as isize));
    let right = map(preceded(tag("R "), i64), |m| Move::Right(m as isize));

    let row_parser = alt((up, down, left, right));

    let res = separated_list1(newline, row_parser)(i)?;

    Ok(res)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const INPUT_2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn aoc9_parse() {
        let (_, moves) = super::parse(INPUT).unwrap();
        assert_eq!(moves.len(), 8);
    }
    #[test]
    fn aoc9_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 13);
    }

    #[test]
    fn aoc9_run_2() {
        assert_eq!(super::run_2(INPUT).unwrap(), 1);
        assert_eq!(super::run_2(INPUT_2).unwrap(), 36);
    }
}
