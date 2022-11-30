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
fn explode(s: &mut SnailFish) {
    fn explode_inner(s: &mut SnailFish, d: usize) -> Option<SnailFish> {
        fn is_pair(s: &SnailFish) -> bool {
            matches!(s, SnailFish::Pair(_))
        }

        fn update_left(s: &mut SnailFish, replacement: &mut SnailFish) {
            if let SnailFish::Pair(rp) = replacement {
                if let SnailFish::Num(rep) = &mut rp.0 {
                    if *rep > 0 {
                        if let SnailFish::Num(val) = s {
                            *val += *rep;
                            *rep = 0;
                        }
                    }
                }
            }
        }

        fn update_right(s: &mut SnailFish, replacement: &mut SnailFish) {
            if let SnailFish::Pair(rp) = replacement {
                if let SnailFish::Num(rep) = &mut rp.1 {
                    if *rep > 0 {
                        if let SnailFish::Num(val) = s {
                            *val += *rep;
                            *rep = 0;
                        }
                    }
                }
            }
        }

        match s {
            SnailFish::Pair(p) => {
                if d == 4 {
                    // Can we explode the left hand side?
                    if is_pair(&p.0) {
                        let mut rep = SnailFish::Num(0);
                        std::mem::swap(&mut p.0, &mut rep);

                        dbg! {&rep};
                        update_right(&mut p.1, &mut rep);

                        return Some(rep);
                    }
                    if is_pair(&p.1) {
                        let mut rep = SnailFish::Num(0);
                        std::mem::swap(&mut p.1, &mut rep);
                        dbg! {&rep};
                        update_left(&mut p.0, &mut rep);
                        return Some(rep);
                    }
                    None
                } else {
                    let l = explode_inner(&mut p.0, d + 1);
                    match l {
                        None => explode_inner(&mut p.1, d + 1),
                        Some(mut rep) => {
                            update_left(s, &mut rep);
                            update_right(s, &mut rep);
                            Some(rep)
                        }
                    }
                }
            }
            _ => None,
        }
    }

    explode_inner(s, 1);
}

fn split(v: usize) -> SnailFish {
    let e = v % 2;
    SnailFish::Pair(Box::new((SnailFish::Num(v / 2), SnailFish::Num(v / 2 + e))))
}

fn magnitude(s: &SnailFish) -> usize {
    match s {
        SnailFish::Num(s) => *s,
        SnailFish::Pair(p) => 3 * magnitude(&p.0) + 2 * magnitude(&p.1),
    }
}

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
    fn aoc18_explode() {
        let (_, mut a) = super::parse("[[[[[9,8],1],2],3],4]").unwrap();
        let (_, b) = super::parse("[[[[0,9],2],3],4]").unwrap();
        super::explode(&mut a);
        assert_eq!(a, b);

        let (_, mut a) = super::parse("[7,[6,[5,[4,[3,2]]]]]").unwrap();
        let (_, b) = super::parse("[7,[6,[5,[7,0]]]]").unwrap();
        super::explode(&mut a);
        assert_eq!(a, b);

        let (_, mut a) = super::parse("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]").unwrap();
        let (_, b) = super::parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]").unwrap();
        super::explode(&mut a);
        // assert_eq!(a, b);
    }

    #[test]
    fn aoc18_magnitude() {
        let (_, a) = super::parse("[9,1]").unwrap();
        assert_eq!(super::magnitude(&a), 29);

        let (_, a) = super::parse("[[9,1],[1,9]]").unwrap();
        assert_eq!(super::magnitude(&a), 129);

        let (_, a) = super::parse("[[1,2],[[3,4],5]]").unwrap();
        assert_eq!(super::magnitude(&a), 143);

        let (_, a) = super::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").unwrap();
        assert_eq!(super::magnitude(&a), 1384);

        let (_, a) = super::parse("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").unwrap();
        assert_eq!(super::magnitude(&a), 3488);
    }

    #[test]
    fn aoc18_split() {
        assert_eq!(super::split(10), super::parse("[5,5]").unwrap().1);
        assert_eq!(super::split(11), super::parse("[5,6]").unwrap().1);
        assert_eq!(super::split(12), super::parse("[6,6]").unwrap().1);
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
