pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day1.txt")?;
    let input = parse(&input)?;

    println!("1:1 - {}", run_1(&input)?);
    println!("3:2 - {}", run_2(&input)?);

    Ok(())
}

fn parse(input: &str) -> anyhow::Result<Vec<usize>> {
    let (_, res) = nom::multi::separated_list1(
        nom::character::complete::newline,
        crate::helper::uval::<usize>,
    )(input)
    .map_err(|e| e.to_owned())?;
    Ok(res)
}

fn run_1(input: &[usize]) -> anyhow::Result<usize> {
    let (_, cnt) = input.iter().fold((usize::MAX, 0), |(prev, cnt), cur| {
        if &prev < cur {
            (*cur, cnt + 1)
        } else {
            (*cur, cnt)
        }
    });
    Ok(cnt)
}

fn run_2(input: &[usize]) -> anyhow::Result<usize> {
    let (_, cnt) =
        input
            .windows(3)
            .map(|v| v.iter().sum())
            .fold((usize::MAX, 0), |(prev, cnt), cur| {
                if prev < cur {
                    (cur, cnt + 1)
                } else {
                    (cur, cnt)
                }
            });
    Ok(cnt)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "199
200
208
210
200
207
240
269
260
263";

    #[test]
    fn aoc1_parse() {
        let result = super::parse(INPUT).unwrap();
        assert_eq!(result.len(), 10);
        assert_eq!(result[0], 199);
        assert_eq!(result[9], 263);
    }

    #[test]
    fn aoc1_run_1() {
        let input = super::parse(INPUT).unwrap();
        assert_eq!(super::run_1(&input).unwrap(), 7);
    }

    #[test]
    fn aoc1_run_2() {
        let input = super::parse(INPUT).unwrap();
        assert_eq!(super::run_2(&input).unwrap(), 5);
    }
}
