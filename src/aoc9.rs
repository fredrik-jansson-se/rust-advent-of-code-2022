use std::fs;

pub fn run() -> anyhow::Result<()> {
    let input = fs::read_to_string("day9.txt").unwrap();
    println!("day9-1: {}", run_1(&input)?);
    println!("day9-2: {}", run_2(&input)?);
    Ok(())
}

fn run_1(input: &str) -> anyhow::Result<isize> {
    let map = parse(input)?;

    let get_val = |(row, col): &(isize, isize)| {
        if *row >= 0
            && *row < (map.len() as isize)
            && *col >= 0
            && *col < (map[*row as usize].len() as isize)
        {
            Some(map[*row as usize][*col as usize])
        } else {
            None
        }
    };

    let mut risk_level = 0;
    for (r, row) in map.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            let min_surround = [
                (r as isize - 1, c as isize),
                (r as isize + 1, c as isize),
                (r as isize, c as isize - 1),
                (r as isize, c as isize + 1),
            ]
            .iter()
            .filter_map(get_val)
            .min()
            .unwrap_or(isize::MAX);
            if *col < min_surround {
                risk_level += col + 1;
            }
        }
    }

    Ok(risk_level)
}

fn run_2(input: &str) -> anyhow::Result<usize> {
    let map = parse(input)?;

    let get_val = |(row, col): &(isize, isize)| {
        if *row >= 0
            && *row < (map.len() as isize)
            && *col >= 0
            && *col < (map[*row as usize].len() as isize)
        {
            Some(map[*row as usize][*col as usize])
        } else {
            None
        }
    };

    let mut basin_points = std::collections::HashSet::new();

    for (r, row) in map.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            let min_surround = [
                (r as isize - 1, c as isize),
                (r as isize + 1, c as isize),
                (r as isize, c as isize - 1),
                (r as isize, c as isize + 1),
            ]
            .iter()
            .filter_map(get_val)
            .min()
            .unwrap_or(isize::MAX);
            if *col < min_surround {
                basin_points.insert((r, c));
            }
        }
    }

    let is_nbr = |(row, col): &(isize, isize)| {
        if *row >= 0
            && *row < (map.len() as isize)
            && *col >= 0
            && *col < (map[*row as usize].len() as isize)
        {
            map[*row as usize][*col as usize] < 9
        } else {
            false
        }
    };

    let mut basin_sizes: Vec<usize> = Vec::new();
    while let Some(pt) = basin_points.iter().next().cloned() {
        let mut searched = std::collections::HashSet::new();
        let mut to_search = vec![pt];
        while let Some(pt) = to_search.pop() {
            searched.insert(pt);
            basin_points.remove(&pt);
            let r = pt.0;
            let c = pt.1;
            for n in [
                (r as isize - 1, c as isize),
                (r as isize + 1, c as isize),
                (r as isize, c as isize - 1),
                (r as isize, c as isize + 1),
            ]
            .iter()
            .filter(|c| is_nbr(c))
            {
                let n = (n.0 as usize, n.1 as usize);
                if !searched.contains(&n) {
                    to_search.push((n.0 as usize, n.1 as usize));
                }
            }
        }

        basin_sizes.push(searched.len());
    }

    basin_sizes.sort_unstable();
    // Multiply the top three sizes
    Ok(basin_sizes.iter().rev().take(3).product())
}

fn parse(i: &str) -> anyhow::Result<Vec<Vec<isize>>> {
    let mut res = Vec::new();
    for line in i.lines() {
        let mut row: Vec<isize> = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap() as isize);
        }
        res.push(row);
    }
    Ok(res)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn aoc9_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 15);
    }

    #[test]
    fn aoc9_run_2() {
        assert_eq!(super::run_2(INPUT).unwrap(), 1134);
    }
}
