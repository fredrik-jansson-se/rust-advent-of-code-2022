use std::fs;

pub fn run() -> anyhow::Result<()> {
    let input = fs::read_to_string("day6.txt").unwrap();
    println!("6:1: {}", run_1(&input)?);
    println!("6:2: {}", run_2(&input)?);
    Ok(())
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let input: Vec<_> = input.chars().collect();

    for idx in 4..input.len() {
        let sub = &input[(idx - 4)..idx];
        if sub[0] != sub[1]
            && sub[0] != sub[2]
            && sub[0] != sub[3]
            && sub[1] != sub[2]
            && sub[1] != sub[3]
            && sub[2] != sub[3]
        {
            return Ok(idx);
        }
    }
    Ok(0)
}

fn run_2(input: &str) -> anyhow::Result<usize> {
    let input: Vec<_> = input.chars().collect();

    for idx in 14..input.len() {
        let sub: std::collections::HashSet<char> = input[(idx - 14)..idx].iter().copied().collect();
        if sub.len() == 14 {
            return Ok(idx);
        }
    }
    Ok(0)
}

#[cfg(test)]
mod tests {
    const INPUT_1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const INPUT_2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const INPUT_3: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const INPUT_4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const INPUT_5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn aoc6_run_1() {
        assert_eq!(super::run_1(INPUT_1).unwrap(), 7);
        assert_eq!(super::run_1(INPUT_2).unwrap(), 5);
        assert_eq!(super::run_1(INPUT_3).unwrap(), 6);
        assert_eq!(super::run_1(INPUT_4).unwrap(), 10);
        assert_eq!(super::run_1(INPUT_5).unwrap(), 11);
    }

    #[test]
    fn aoc6_run_2() {
        assert_eq!(super::run_2(INPUT_1).unwrap(), 19);
        assert_eq!(super::run_2(INPUT_2).unwrap(), 23);
        assert_eq!(super::run_2(INPUT_3).unwrap(), 23);
        assert_eq!(super::run_2(INPUT_4).unwrap(), 29);
        assert_eq!(super::run_2(INPUT_5).unwrap(), 26);
    }
}
