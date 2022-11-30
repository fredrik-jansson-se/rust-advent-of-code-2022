pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day2.txt")?;
    let input = parse(&input)?;
    println!("2:1: {}", run_1(&input)?);
    println!("2:2: {}", run_2(&input)?);
    Ok(())
}

fn run_1(input: &[Command]) -> anyhow::Result<isize> {
    let (x, d) = input.iter().fold((0, 0), |(x, d), cmd| match cmd {
        Command::Forward(v) => (x + v, d),
        Command::Down(v) => (x, d + v),
        Command::Up(v) => (x, d - v),
    });
    Ok(x * d)
}

fn run_2(input: &[Command]) -> anyhow::Result<isize> {
    let (x, d, _aim) = input.iter().fold((0, 0, 0), |(x, d, aim), cmd| match cmd {
        Command::Forward(v) => (x + v, d + aim * v, aim),
        Command::Down(v) => (x, d, aim + v),
        Command::Up(v) => (x, d, aim - v),
    });
    Ok(x * d)
}

#[derive(Debug, PartialEq)]
enum Command {
    Forward(isize),
    Down(isize),
    Up(isize),
}

fn parse(i: &str) -> anyhow::Result<Vec<Command>> {
    let fwd = nom::combinator::map(
        nom::sequence::preceded(
            nom::sequence::pair(
                nom::bytes::complete::tag("forward"),
                nom::character::complete::space0,
            ),
            crate::helper::ival::<isize>,
        ),
        Command::Forward,
    );
    let down = nom::combinator::map(
        nom::sequence::preceded(
            nom::sequence::pair(
                nom::bytes::complete::tag("down"),
                nom::character::complete::space0,
            ),
            crate::helper::ival::<isize>,
        ),
        Command::Down,
    );
    let up = nom::combinator::map(
        nom::sequence::preceded(
            nom::sequence::pair(
                nom::bytes::complete::tag("up"),
                nom::character::complete::space0,
            ),
            crate::helper::ival::<isize>,
        ),
        Command::Up,
    );
    let (_, res) = nom::multi::separated_list1(
        nom::character::complete::newline,
        nom::branch::alt((fwd, down, up)),
    )(i)
    .map_err(|e| e.to_owned())?;
    Ok(res)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn aoc2_parse() {
        let input = super::parse(INPUT).unwrap();

        assert_eq!(input.len(), 6);
        assert_eq!(input[0], super::Command::Forward(5));
        assert_eq!(input[1], super::Command::Down(5));
    }

    #[test]
    fn aoc2_run_1() {
        let input = super::parse(INPUT).unwrap();
        assert_eq!(super::run_1(&input).unwrap(), 150);
    }

    #[test]
    fn aoc2_run_2() {
        let input = super::parse(INPUT).unwrap();
        assert_eq!(super::run_2(&input).unwrap(), 900);
    }
}
