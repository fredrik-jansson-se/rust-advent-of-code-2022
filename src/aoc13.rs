use std::{collections::HashSet, fs};

pub fn run() -> anyhow::Result<()> {
    let input = fs::read_to_string("day13.txt").unwrap();
    println!("day13-1: {}", run_1(&input)?);
    println!("day13-2: \n{}", run_2(&input)?);
    Ok(())
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let (mut coords, folds) = parse(input)?;

    if let Some(fold) = folds.get(0) {
        match fold {
            Fold::X(x) => {
                let outsiders = coords
                    .iter()
                    .filter(|(cx, _)| cx > x)
                    .cloned()
                    .collect::<Vec<_>>();
                for o in outsiders {
                    coords.remove(&o);
                    let dx = o.0 - x;
                    coords.insert((o.0 - 2 * dx, o.1));
                }
            }
            Fold::Y(y) => {
                let outsiders = coords
                    .iter()
                    .filter(|(_, cy)| cy > y)
                    .cloned()
                    .collect::<Vec<_>>();
                for o in outsiders {
                    coords.remove(&o);
                    let dy = o.1 - y;
                    coords.insert((o.0, o.1 - 2 * dy));
                }
            }
        }
    }
    Ok(coords.len())
}

fn run_2(input: &str) -> anyhow::Result<String> {
    let (mut coords, folds) = parse(input)?;

    for fold in folds.iter() {
        match fold {
            Fold::X(x) => {
                let outsiders = coords
                    .iter()
                    .filter(|(cx, _)| cx > x)
                    .cloned()
                    .collect::<Vec<_>>();
                for o in outsiders {
                    coords.remove(&o);
                    let dx = o.0 - x;
                    coords.insert((o.0 - 2 * dx, o.1));
                }
            }
            Fold::Y(y) => {
                let outsiders = coords
                    .iter()
                    .filter(|(_, cy)| cy > y)
                    .cloned()
                    .collect::<Vec<_>>();
                for o in outsiders {
                    coords.remove(&o);
                    let dy = o.1 - y;
                    coords.insert((o.0, o.1 - 2 * dy));
                }
            }
        }
    }

    Ok(to_string(&coords))
}

fn to_string(coords: &HashSet<Coord>) -> String {
    let max_x = *coords.iter().map(|(x, _)| x).max().unwrap() + 1;
    let max_y = *coords.iter().map(|(_, y)| y).max().unwrap() + 1;
    let mut res: Vec<String> = Vec::with_capacity(max_y);
    for y in 0..max_y {
        let mut row = Vec::with_capacity(max_x);
        for x in 0..max_x {
            if coords.contains(&(x, y)) {
                row.push("#");
            } else {
                row.push(".");
            }
        }
        res.push(row.join(""));
    }
    res.join("\n")
}

type Coord = (usize, usize);
#[derive(Debug, PartialEq)]
enum Fold {
    X(usize),
    Y(usize),
}

fn parse(i: &str) -> anyhow::Result<(HashSet<Coord>, Vec<Fold>)> {
    let coords = nom::multi::separated_list1(
        nom::character::complete::newline,
        nom::sequence::separated_pair(
            crate::helper::uval::<usize>,
            nom::bytes::complete::tag(","),
            crate::helper::uval::<usize>,
        ),
    );

    let fold_y = nom::combinator::map(
        nom::sequence::preceded(
            nom::bytes::complete::tag("fold along y="),
            crate::helper::uval::<usize>,
        ),
        Fold::Y,
    );
    let fold_x = nom::combinator::map(
        nom::sequence::preceded(
            nom::bytes::complete::tag("fold along x="),
            crate::helper::uval::<usize>,
        ),
        Fold::X,
    );

    let folds = nom::multi::separated_list1(
        nom::character::complete::newline,
        nom::branch::alt((fold_x, fold_y)),
    );

    let (_, (coords, folds)) = nom::sequence::separated_pair(
        coords,
        nom::multi::many1(nom::character::complete::newline),
        folds,
    )(i)
    .map_err(|e| e.to_owned())?;

    let coords = coords.into_iter().collect();
    Ok((coords, folds))
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn aoc13_fold() {
        let (coords, folds) = super::parse(INPUT).unwrap();
        assert_eq!(coords.len(), 18);
        assert_eq!(folds[0], super::Fold::Y(7));
        assert_eq!(folds[1], super::Fold::X(5));
    }

    #[test]
    fn aoc13_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 17);
    }

    #[test]
    fn aoc13_run_2() {
        let out = super::run_2(INPUT).unwrap();
        println!("{}", out);
    }
}
