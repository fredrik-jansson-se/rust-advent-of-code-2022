use std::fs;

pub fn run() -> anyhow::Result<()> {
    let input = fs::read_to_string("day11.txt").unwrap();
    println!("day11-1: {}", run_1(&input)?);
    println!("day11-2: {}", run_2(&input)?);
    Ok(())
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let mut octs = Vec::new();
    for line in input.lines() {
        let row = line
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<_>>();
        octs.push(row);
    }

    let mut num_flashes = 0;
    for _ in 0..100 {
        let mut flashed = std::collections::HashSet::new();
        for (r, row) in octs.iter_mut().enumerate() {
            for (c, o) in row.iter_mut().enumerate() {
                *o += 1;
                if *o == 10 {
                    num_flashes += 1;
                    flashed.insert((r, c));
                }
            }
        }

        while let Some((r, c)) = flashed.iter().next().cloned() {
            flashed.remove(&(r, c));
            let nbrs = [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ]
            .iter()
            .filter_map(|(dr, dc)| {
                let r = r as i32 + dr;
                let c = c as i32 + dc;
                if (0..10).contains(&r) && (0..10).contains(&c) {
                    Some((r as usize, c as usize))
                } else {
                    None
                }
            });
            for (r, c) in nbrs {
                let o = octs.get_mut(r).unwrap().get_mut(c).unwrap();
                *o += 1;
                if *o == 10 {
                    num_flashes += 1;
                    flashed.insert((r, c));
                }
            }
        }
        for row in octs.iter_mut() {
            for o in row.iter_mut() {
                if *o > 9 {
                    *o = 0;
                }
            }
        }
    }

    Ok(num_flashes)
}

fn run_2(input: &str) -> anyhow::Result<usize> {
    let mut octs = Vec::new();
    for line in input.lines() {
        let row = line
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<_>>();
        octs.push(row);
    }

    let mut step = 0;
    loop {
        let mut all_flashed = std::collections::HashSet::new();
        let mut flashed = std::collections::HashSet::new();
        for (r, row) in octs.iter_mut().enumerate() {
            for (c, o) in row.iter_mut().enumerate() {
                *o += 1;
                if *o == 10 {
                    flashed.insert((r, c));
                    all_flashed.insert((r, c));
                }
            }
        }

        while let Some((r, c)) = flashed.iter().next().cloned() {
            flashed.remove(&(r, c));
            let nbrs = [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ]
            .iter()
            .filter_map(|(dr, dc)| {
                let r = r as i32 + dr;
                let c = c as i32 + dc;
                if (0..10).contains(&r) && (0..10).contains(&c) {
                    Some((r as usize, c as usize))
                } else {
                    None
                }
            });
            for (r, c) in nbrs {
                let o = octs.get_mut(r).unwrap().get_mut(c).unwrap();
                *o += 1;
                if *o == 10 {
                    flashed.insert((r, c));
                    all_flashed.insert((r, c));
                }
            }
        }
        for row in octs.iter_mut() {
            for o in row.iter_mut() {
                if *o > 9 {
                    *o = 0;
                }
            }
        }
        step += 1;
        if all_flashed.len() == 100 {
            return Ok(step);
        }
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn aoc11_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 1656);
    }

    #[test]
    fn aoc11_run_2() {
        assert_eq!(super::run_2(INPUT).unwrap(), 195);
    }
}
