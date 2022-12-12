use crate::{Input, PResult};

pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day2.txt")?;
    println!("2:1: {}", run_1(&input)?);
    println!("2:2: {}", run_2(&input)?);
    Ok(())
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let (_, plays) = parse_1(input).map_err(|e| anyhow::anyhow!("{e}"))?;

    let res: usize = plays.iter().map(|(a, b)| b.play(a)).sum();
    Ok(res)
}

fn run_2(input: &str) -> anyhow::Result<usize> {
    let (_, plays) = parse_2(input).map_err(|e| anyhow::anyhow!("{e}"))?;
    let plays = plays.iter().map(|(o, d)| (o, d.play(o)));
    let res: usize = plays.map(|(a, b)| b.play(a)).sum();
    Ok(res)
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq)]
enum Desired {
    Win,
    Draw,
    Loss,
}

impl Desired {
    fn play(&self, o: &Play) -> Play {
        match self {
            Self::Draw => *o,
            Self::Win => match o {
                Play::Rock => Play::Paper,
                Play::Paper => Play::Scissors,
                Play::Scissors => Play::Rock,
            },
            Self::Loss => match o {
                Play::Rock => Play::Scissors,
                Play::Paper => Play::Rock,
                Play::Scissors => Play::Paper,
            },
        }
    }
}

impl Play {
    fn wins(&self, o: &Self) -> std::cmp::Ordering {
        if self == o {
            std::cmp::Ordering::Equal
        } else {
            match (self, o) {
                (Self::Rock, Self::Scissors) => std::cmp::Ordering::Greater,
                (Self::Paper, Self::Rock) => std::cmp::Ordering::Greater,
                (Self::Scissors, Self::Paper) => std::cmp::Ordering::Greater,
                _ => std::cmp::Ordering::Less,
            }
        }
    }

    fn score(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn play(&self, o: &Self) -> usize {
        let play_score = self.score();
        let game_score = match self.wins(o) {
            std::cmp::Ordering::Less => 0,
            std::cmp::Ordering::Equal => 3,
            std::cmp::Ordering::Greater => 6,
        };

        play_score + game_score
    }
}

fn parse_play_1(i: Input) -> PResult<Play> {
    use nom::{branch::alt, character::complete::char, combinator::map};
    let rock_1 = map(char('A'), |_| Play::Rock);
    let rock_2 = map(char('X'), |_| Play::Rock);
    let paper_1 = map(char('B'), |_| Play::Paper);
    let paper_2 = map(char('Y'), |_| Play::Paper);
    let scissors_1 = map(char('C'), |_| Play::Scissors);
    let scissors_2 = map(char('Z'), |_| Play::Scissors);
    let play = alt((rock_1, rock_2, paper_1, paper_2, scissors_1, scissors_2))(i)?;
    Ok(play)
}

fn parse_1(i: Input) -> PResult<Vec<(Play, Play)>> {
    let round =
        nom::sequence::separated_pair(parse_play_1, nom::character::complete::space1, parse_play_1);

    let rounds = nom::multi::separated_list1(nom::character::complete::newline, round)(i)?;
    Ok(rounds)
}

fn parse_play_2(i: Input) -> PResult<Play> {
    use nom::{branch::alt, character::complete::char, combinator::map};
    let rock_1 = map(char('A'), |_| Play::Rock);
    let paper_1 = map(char('B'), |_| Play::Paper);
    let scissors_1 = map(char('C'), |_| Play::Scissors);
    let play = alt((rock_1, paper_1, scissors_1))(i)?;
    Ok(play)
}

fn parse_desired(i: Input) -> PResult<Desired> {
    use nom::{branch::alt, character::complete::char, combinator::map};
    let draw = map(char('Y'), |_| Desired::Draw);
    let win = map(char('Z'), |_| Desired::Win);
    let loss = map(char('X'), |_| Desired::Loss);
    let play = alt((draw, win, loss))(i)?;
    Ok(play)
}

fn parse_2(i: Input) -> PResult<Vec<(Play, Desired)>> {
    let round = nom::sequence::separated_pair(
        parse_play_2,
        nom::character::complete::space1,
        parse_desired,
    );

    let rounds = nom::multi::separated_list1(nom::character::complete::newline, round)(i)?;
    Ok(rounds)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn aoc2_parse() {
        let (_, input) = super::parse_1(INPUT).unwrap();

        assert_eq!(input.len(), 3);
    }

    #[test]
    fn aoc2_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 15);
    }

    #[test]
    fn aoc2_run_2() {
        assert_eq!(super::run_2(INPUT).unwrap(), 12);
    }
}
