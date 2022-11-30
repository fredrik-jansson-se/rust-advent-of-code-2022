use std::fs;

pub fn run() -> anyhow::Result<()> {
    let input = fs::read_to_string("day3.txt")?;

    println!("3:1 - {}", run_1(&input)?);
    println!("3:2 - {}", run_2(&input)?);
    Ok(())
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let mut one_cnt = vec![0usize; 20];
    let mut len = 0;

    for line in input.lines() {
        one_cnt.truncate(line.len());
        len += 1;
        // let mut idx = 0;
        for (idx, v) in line.chars().rev().enumerate() {
            if v == '1' {
                one_cnt[idx] += 1;
            }
            // idx += 1;
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;

    for (idx, i) in one_cnt.iter().enumerate() {
        if *i > (len / 2) {
            gamma += 1 << idx;
        } else {
            epsilon += 1 << idx;
        }
    }
    Ok(epsilon * gamma)
}

fn oxygen_rating(input: Vec<Vec<u32>>, idx: usize) -> Vec<u32> {
    if input.len() == 1 {
        return input[0].clone();
    }

    let num_ones = input.iter().filter(|line| line[idx] == 1).count();
    let filter = (num_ones >= (input.len() - num_ones)) as u32;
    let input = input
        .into_iter()
        .filter(|line| line[idx] == filter)
        .collect();
    oxygen_rating(input, idx + 1)
}

fn co2_rating(input: Vec<Vec<u32>>, idx: usize) -> Vec<u32> {
    if input.len() == 1 {
        return input[0].clone();
    }

    let num_ones = input.iter().filter(|line| line[idx] == 1).count();
    let filter = (num_ones < (input.len() - num_ones)) as u32;
    let input = input
        .into_iter()
        .filter(|line| line[idx] == filter)
        .collect();
    co2_rating(input, idx + 1)
}

fn run_2(input: &str) -> anyhow::Result<u32> {
    let input: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u32)
                .collect()
        })
        .collect();

    let oxy = oxygen_rating(input.clone(), 0);
    let mut oxy_val = 0;
    for o in oxy.iter() {
        oxy_val = (oxy_val << 1) + o;
    }

    let co2 = co2_rating(input, 0);
    let mut co2_val = 0;
    for c in co2.iter() {
        co2_val = (co2_val << 1) + c;
    }

    Ok(co2_val * oxy_val)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn aoc3_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 198);
    }

    #[test]
    fn aoc3_run_2() {
        assert_eq!(super::run_2(INPUT).unwrap(), 230);
    }
}
