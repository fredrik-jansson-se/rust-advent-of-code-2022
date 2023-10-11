use std::fs;

pub fn run() -> anyhow::Result<()> {
    let input = fs::read_to_string("day8.txt")?;
    println!("day8-1: {}", run_1(&input)?);
    println!("day8-2: {}", run_2(&input)?);
    Ok(())
}

type Map = Vec<Vec<isize>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VisibleFrom {
    Top,
    Bottom,
    Right,
    Left,
}

#[derive(Debug, Clone)]
struct Tree {
    height: isize,
    visible_from: Vec<VisibleFrom>,
}

impl Tree {
    fn new(height: isize) -> Self {
        Self {
            height,
            visible_from: vec![
                VisibleFrom::Top,
                VisibleFrom::Bottom,
                VisibleFrom::Left,
                VisibleFrom::Right,
            ],
        }
    }
    fn is_visible(&self) -> bool {
        !self.visible_from.is_empty()
    }
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let (_, map) = parse(input).map_err(|e| anyhow::anyhow!("{e}"))?;

    let mut visible_map = Vec::with_capacity(map.len());
    for row in map.iter() {
        let mut vrow = Vec::with_capacity(row.len());
        for col in row.iter() {
            vrow.push(Tree::new(*col));
        }
        visible_map.push(vrow);
    }

    let mut map = visible_map;
    for r_i in 0..map.len() {
        let row = &mut map[r_i];
        let row_len = row.len();
        let mut max_from_left = -1;
        let mut max_from_right = -1;
        for c_i in 0..row_len {
            let cur_tree = &mut row[c_i];
            if cur_tree.height <= max_from_left {
                cur_tree.visible_from.retain(|v| *v != VisibleFrom::Left);
            }
            max_from_left = max_from_left.max(cur_tree.height);

            let cur_tree = &mut row[row_len - 1 - c_i];
            if cur_tree.height <= max_from_right {
                cur_tree.visible_from.retain(|v| *v != VisibleFrom::Right);
            }
            max_from_right = max_from_right.max(cur_tree.height);
        }
    }

    for c_i in 0..map[0].len() {
        let height = map.len();
        let mut max_from_top = -1;
        let mut max_from_bottom = -1;
        for r_i in 0..height {
            let cur_tree = &mut map[r_i][c_i];
            if cur_tree.height <= max_from_top {
                cur_tree.visible_from.retain(|v| *v != VisibleFrom::Top);
            }
            max_from_top = max_from_top.max(cur_tree.height);

            let cur_tree = &mut map[height - 1 - r_i][c_i];
            if cur_tree.height <= max_from_bottom {
                cur_tree.visible_from.retain(|v| *v != VisibleFrom::Bottom);
            }
            max_from_bottom = max_from_bottom.max(cur_tree.height);
        }
    }

    Ok(map
        .iter()
        .map(|row| row.iter().filter(|tree| tree.is_visible()).count())
        .sum())
}

fn run_2(input: &str) -> anyhow::Result<usize> {
    let (_, _map) = parse(input).map_err(|e| anyhow::anyhow!("{e}"))?;
    todo!()
}

fn parse(i: crate::Input) -> crate::PResult<Map> {
    let mut res = Vec::new();

    for row in i.lines() {
        let res_row = row
            .chars()
            .map(|c| c.to_digit(10).unwrap() as isize)
            .collect();
        res.push(res_row);
    }

    Ok(("", res))
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn aoc8_parse() {
        let (_, map) = super::parse(INPUT).unwrap();

        assert_eq!(map.len(), 5);
        assert_eq!(map.get(0).unwrap().len(), 5);
    }

    #[test]
    fn aoc8_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 21);
    }

    #[test]
    fn aoc8_run_2() {
        assert_eq!(super::run_2(INPUT).unwrap(), 5353);
    }
}
