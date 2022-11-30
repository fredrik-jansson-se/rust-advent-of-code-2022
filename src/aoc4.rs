use std::fs;

pub fn run() -> anyhow::Result<()> {
    let input = fs::read_to_string("day4.txt").unwrap();
    println!("4:1 {}", run_1(&input)?);
    println!("4:2 {}", run_2(&input)?);
    Ok(())
}

fn check_board(input: usize, board: &mut [Vec<usize>]) -> bool {
    let mut found = None;
    for (r_num, row) in board.iter_mut().enumerate() {
        for (c_num, val) in row.iter_mut().enumerate() {
            if *val == input {
                *val = usize::MAX;
                found = Some((r_num, c_num));
                break;
            }
        }
    }

    if let Some((row, col)) = found {
        // Check if winning row
        board[row].iter().all(|v| *v == usize::MAX)
            || board.iter().map(|row| row[col]).all(|v| v == usize::MAX)
    } else {
        false
    }
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let (input, mut boards) = parse(input)?;

    for i in input.iter() {
        for board in boards.iter_mut() {
            if check_board(*i, board) {
                let sum: usize = board.iter().flatten().filter(|v| **v != usize::MAX).sum();
                return Ok(i * sum);
            }
        }
    }
    Ok(0)
}

fn run_2(input: &str) -> anyhow::Result<usize> {
    let (input, mut boards) = parse(input)?;

    while boards.len() > 1 {
        for i in input.iter() {
            boards.retain(|b| !b.is_empty());
            let boards_left = boards.len();
            for board in boards.iter_mut() {
                if check_board(*i, board) {
                    if boards_left == 1 {
                        let sum: usize = board.iter().flatten().filter(|v| **v != usize::MAX).sum();
                        return Ok(i * sum);
                    } else {
                        board.truncate(0);
                    }
                }
            }
        }
    }
    Ok(0)
}

type Board = Vec<Vec<usize>>;
fn parse(i: &str) -> anyhow::Result<(Vec<usize>, Vec<Board>)> {
    // get input numbers
    let (i, input) =
        nom::multi::separated_list1(nom::bytes::complete::tag(","), crate::helper::uval)(i)
            .map_err(|e| e.to_owned())?;

    let (i, _) = nom::multi::many1(nom::character::complete::newline)(i)
        .map_err(|e: nom::Err<nom::error::Error<&str>>| e.to_owned())?;

    let parse_row = |i| {
        nom::multi::count(
            nom::sequence::preceded(nom::character::complete::space0, crate::helper::uval),
            5,
        )(i)
    };

    let board = nom::multi::count(
        nom::sequence::terminated(
            parse_row,
            nom::combinator::opt(nom::character::complete::newline),
        ),
        5,
    );

    let (_i, boards) = nom::multi::separated_list1(nom::character::complete::newline, board)(i)
        .map_err(|e| e.to_owned())?;

    Ok((input, boards))
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#;

    #[test]
    fn aoc4_parse() {
        let (_input, boards) = super::parse(INPUT).unwrap();
        assert_eq!(boards.len(), 3);
    }

    #[test]
    fn aoc4_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 4512)
    }

    #[test]
    fn aoc4_run_2() {
        assert_eq!(super::run_2(INPUT).unwrap(), 1924)
    }
}
