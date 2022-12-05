use std::fs;

pub fn run() -> anyhow::Result<()> {
    let input = fs::read_to_string("day5.txt").unwrap();

    println!("5:1 {}", run_1(&input)?);
    println!("5:2 {}", run_2(&input)?);
    Ok(())
}

fn run_1(input: &str) -> anyhow::Result<String> {
    let (_, mut game) = parse(input).map_err(|e| anyhow::anyhow!("{e}"))?;

    for mv in game.moves {
        let (num, from, to) = mv;

        for _ in 0..num {
            let item = game.stack[from - 1].pop().unwrap();
            game.stack[to - 1].push(item);
        }
    }

    let mut res = String::new();
    for s in game.stack.iter() {
        if let StackItem::Crate(c) = s[s.len() - 1] {
            res += &format!("{c}");
        }
    }
    Ok(res)
}

fn run_2(input: &str) -> anyhow::Result<String> {
    let (_, mut game) = parse(input).map_err(|e| anyhow::anyhow!("{e}"))?;

    for mv in game.moves {
        let (num, from, to) = mv;

        let mut tmp = Vec::new();
        for _ in 0..num {
            let item = game.stack[from - 1].pop().unwrap();
            tmp.push(item);
        }

        for t in tmp.iter().rev() {
            game.stack[to - 1].push(*t);
        }
    }

    let mut res = String::new();
    for s in game.stack.iter() {
        if let StackItem::Crate(c) = s[s.len() - 1] {
            res += &format!("{c}");
        }
    }
    Ok(res)
}

type Input<'a> = &'a str;
type PResult<'a, T> = nom::IResult<Input<'a>, T, nom::error::VerboseError<Input<'a>>>;

#[derive(Debug, Clone, Copy, PartialEq)]
enum StackItem {
    Crate(char),
    Empty,
}

fn parse_stack_item(i: Input) -> PResult<StackItem> {
    let empty = nom::combinator::map(
        nom::multi::many_m_n(3, 3, nom::character::complete::char(' ')),
        |_| StackItem::Empty,
    );
    let c = nom::combinator::map(
        nom::sequence::delimited(
            nom::bytes::complete::tag("["),
            nom::character::complete::anychar,
            nom::bytes::complete::tag("]"),
        ),
        StackItem::Crate,
    );

    let r = nom::branch::alt((empty, c))(i)?;
    Ok(r)
}

fn parse_stack_horiz(i: Input) -> PResult<Vec<StackItem>> {
    let r = nom::multi::separated_list0(nom::bytes::complete::tag(" "), parse_stack_item)(i)?;
    Ok(r)
}

fn parse_move(i: Input) -> PResult<(usize, usize, usize)> {
    use nom::bytes::complete::tag;

    let (i, _) = tag("move ")(i)?;
    let (i, num) = nom::character::complete::u32(i)?;
    let (i, _) = tag(" from ")(i)?;
    let (i, from) = nom::character::complete::u32(i)?;
    let (i, _) = tag(" to ")(i)?;
    let (i, to) = nom::character::complete::u32(i)?;
    Ok((i, (num as usize, from as usize, to as usize)))
}

struct Game {
    stack: Vec<Vec<StackItem>>,
    moves: Vec<(usize, usize, usize)>,
}

fn parse(i: &str) -> PResult<Game> {
    let (i, mut stack) =
        nom::multi::separated_list0(nom::character::complete::newline, parse_stack_horiz)(i)?;
    stack.retain(|row| !row.is_empty());

    // Transform stack from horizontal
    let mut real_stack = Vec::new();
    for row in stack.iter().rev() {
        for (col_idx, item) in row.iter().enumerate() {
            if real_stack.get(col_idx).is_none() {
                real_stack.push(Vec::new());
            }
            if *item != StackItem::Empty {
                let col = &mut real_stack[col_idx];
                col.push(*item);
            }
        }
    }

    let (i, _) = nom::multi::many0(nom::sequence::preceded(
        nom::character::complete::space0,
        nom::character::complete::u32,
    ))(i)?;
    let (i, _) = nom::character::complete::space0(i)?;
    let (i, _) = nom::multi::many1(nom::character::complete::newline)(i)?;

    let (i, moves) = nom::multi::separated_list0(nom::character::complete::newline, parse_move)(i)?;

    Ok((
        i,
        Game {
            stack: real_stack,
            moves,
        },
    ))
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[test]
    fn aoc5_parse() {
        assert_eq!(
            super::parse_stack_item("   ").unwrap().1,
            super::StackItem::Empty
        );
        assert_eq!(
            super::parse_stack_item("[A]").unwrap().1,
            super::StackItem::Crate('A')
        );

        let (_, game) = super::parse(INPUT).unwrap();
        assert_eq!(game.moves.len(), 4);
    }

    #[test]
    fn aoc5_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap().as_str(), "CMZ");
    }
    #[test]
    fn aoc5_run_2() {
        assert_eq!(super::run_2(INPUT).unwrap().as_str(), "MCD");
    }
}
