pub fn run() -> anyhow::Result<()> {
    println!("day21-1: {}", run_1(8, 2)?);
    // println!("day21-2: {}", run_2(&input)?);
    Ok(())
}

struct DeterministicDie {
    cur: usize,
    rolls: usize,
}

impl DeterministicDie {
    fn new() -> Self {
        Self { cur: 0, rolls: 0 }
    }

    fn next(&mut self) -> usize {
        let cur = self.cur;
        self.cur = (self.cur + 1) % 100;
        self.rolls += 1;

        cur + 1
    }
}

fn run_1(mut p1: usize, mut p2: usize) -> anyhow::Result<usize> {
    // Make zero based
    p1 -= 1;
    p2 -= 1;

    let mut p1_score = 0;
    let mut p2_score = 0;
    let mut die = DeterministicDie::new();

    while p2_score < 1000 {
        let m = die.next() + die.next() + die.next();
        p1 += m;
        p1_score += (p1 % 10) + 1;
        if p1_score >= 1000 {
            break;
        }
        let m = die.next() + die.next() + die.next();
        p2 += m;
        p2_score += (p2 % 10) + 1;
    }

    dbg! {(p1_score, p2_score, die.rolls)};
    Ok(p1_score.min(p2_score) * die.rolls)
}

// fn run_2(_input: &str) -> anyhow::Result<usize> {
//     todo!()
// }

#[cfg(test)]
mod tests {
    #[test]
    fn aoc21_run_1() {
        assert_eq!(super::run_1(4, 8).unwrap(), 739785);
    }
}
