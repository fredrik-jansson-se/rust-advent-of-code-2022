pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day18.txt").unwrap();
    println!("18:1: {}", run_1(&input)?);
    println!("18:2: {}", run_2(&input)?);
    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum SnailFish {
    Pair(Box<(SnailFish, SnailFish)>),
    Num(usize),
}

impl SnailFish {
    // fn on_left<F>(&mut self, f: F)
    // where
    //     F: Fn(&SnailFish) -> SnailFish,
    // {
    //     match self {
    //         Self::Pair(p) => {
    //             p.0 = f(&p.0);
    //         }
    //         _ => (),
    //     }
    // }

    // fn on_left_num<F>(&mut self, f: F)
    // where
    //     F: Fn(usize) -> usize,
    // {
    //     let of = |s: &SnailFish| match s {
    //         Self::Num(n) => Self::Num(f(n)),
    //         s => s,
    //     };
    //     self.on_left(of)
    // }

    // fn on_right<F>(&mut self, f: F)
    // where
    //     F: Fn(SnailFish) -> SnailFish,
    // {
    //     match self {
    //         Self::Pair(p) => {
    //             p.1 = f(p.1);
    //         }
    //         _ => (),
    //     }
    // }

    // fn on_right_num<F>(&mut self, f: F)
    // where
    //     F: Fn(usize) -> usize,
    // {
    //     let of = |s: SnailFish| match s {
    //         Self::Num(n) => Self::Num(f(n)),
    //         s => s,
    //     };
    //     self.on_right(of)
    // }
}

type Input<'a> = &'a str;
type PResult<'a> = nom::IResult<Input<'a>, SnailFish, nom::error::VerboseError<Input<'a>>>;

fn parse_num(i: Input) -> PResult {
    use nom::{character::complete::u64, combinator::map};
    map(u64, |n| SnailFish::Num(n as _))(i)
}

fn parse_pair(i: Input) -> PResult {
    use nom::bytes::complete::tag;
    let (i, _) = tag("[")(i)?;

    use nom::sequence::separated_pair;

    let (i, (a, b)) = separated_pair(parse, tag(","), parse)(i)?;

    let (i, _) = tag("]")(i)?;
    Ok((i, SnailFish::Pair(Box::new((a, b)))))
}

fn parse(i: Input) -> PResult {
    use nom::branch::alt;

    alt((parse_num, parse_pair))(i)
}

// fn find_leftmost(s: &mut SnailFish) -> Option<&mut jj>

fn run_1(input: &str) -> anyhow::Result<usize> {
    for line in input.lines() {
        let _ = parse(line).unwrap();
    }
    Ok(0)
}

fn run_2(input: &str) -> anyhow::Result<usize> {
    for line in input.lines() {
        let _ = parse(line).unwrap();
    }
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::SnailFish;
    #[test]
    fn aoc18_parse() {
        assert_eq!(super::parse("12").unwrap().1, SnailFish::Num(12));
        assert_eq!(
            super::parse("[1,2]").unwrap().1,
            SnailFish::Pair(Box::new((SnailFish::Num(1), SnailFish::Num(2))))
        );
    }

    #[test]
    fn aoc18_eval_1() {
        //
    }

    #[test]
    fn aoc18_eval_2() {
        //
    }
}
