use pathfinding::directed::astar::*;
use std::fs;

pub fn run() -> anyhow::Result<()> {
    let input = fs::read_to_string("day15.txt").unwrap();
    println!("day15-1: {}", run_1(&input)?);
    println!("day15-2: {}", run_2(&input)?);
    Ok(())
}

fn solve(map: Vec<Vec<isize>>) -> anyhow::Result<usize> {
    let height = map.len() as isize;
    let width = map[0].len() as isize;

    let get_nbr = |(row, col): (isize, isize)| {
        if (0..height).contains(&row) && (0..width).contains(&col) {
            Some(((row, col), map[row as usize][col as usize]))
        } else {
            None
        }
    };

    let successors = |(row, col): &(isize, isize)| {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            .filter_map(|(dr, dc)| get_nbr((row + dr, col + dc)))
            .collect::<Vec<_>>()
    };

    let goal = (height - 1, width - 1);
    let (_res, cost) = astar(
        &(0, 0),
        successors,
        |(row, col)| ((goal.0 - row) + (col - goal.1)),
        |g| g == &goal,
    )
    .unwrap();
    Ok(cost as usize)
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let map: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as isize)
                .collect::<Vec<_>>()
        })
        .collect();
    solve(map)
}

fn run_2(input: &str) -> anyhow::Result<usize> {
    let map: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as isize)
                .collect::<Vec<_>>()
        })
        .collect();

    solve(expand(map))
}

fn expand(mut map: Vec<Vec<isize>>) -> Vec<Vec<isize>> {
    let old_size = map.len();
    let new_size = map.len() * 5;
    map.reserve(new_size);
    // First expand right
    for (_, row) in map.iter_mut().enumerate() {
        row.resize(new_size, 0);
        for c in old_size..new_size {
            row[c] = row[c - old_size] % 9 + 1;
        }
    }

    for r in old_size..new_size {
        let mut row = vec![0; new_size];
        for (c, value) in row.iter_mut().enumerate() {
            *value = map[r - old_size][c] % 9 + 1;
        }

        map.push(row);
    }

    map
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn aoc15_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 40);
    }

    #[test]
    fn aoc15_run_2() {
        assert_eq!(super::run_2(INPUT).unwrap(), 315);
    }

    #[test]
    fn aoc15_expand() {
        let expanded = super::expand(vec![vec![8]]);
        assert_eq!(
            expanded,
            vec![
                vec![8, 9, 1, 2, 3],
                vec![9, 1, 2, 3, 4],
                vec![1, 2, 3, 4, 5],
                vec![2, 3, 4, 5, 6],
                vec![3, 4, 5, 6, 7]
            ]
        );
    }
}
