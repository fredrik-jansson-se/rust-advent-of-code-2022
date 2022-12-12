use std::fs;

pub fn run() -> anyhow::Result<()> {
    let input = fs::read_to_string("day8.txt")?;
    println!("day8-1: {}", run_1(&input)?);
    println!("day8-2: {}", run_2(&input)?);
    Ok(())
}

type Map = Vec<Vec<usize>>;

fn run_1(input: &str) -> anyhow::Result<usize> {
    let (_, map) = parse(input).map_err(|e| anyhow::anyhow!("{e}"))?;
    let max_rows = map.len();
    let max_cols = map[0].len();

    let mut map_top = map.clone();
    for col in 0..max_cols {
        // let cur_max =
        // for row in 0..max_rows {
        // }
    }
    todo!()
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
            .map(|c| c.to_digit(10).unwrap() as usize)
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
