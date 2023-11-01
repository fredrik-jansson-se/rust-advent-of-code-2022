use std::fs;

pub fn run() -> anyhow::Result<()> {
    let input = fs::read_to_string("day10.txt").unwrap();
    println!("day10-1: {}", run_1(&input)?);
    println!("day10-2:\n{}", run_2(&input)?);
    Ok(())
}

fn evaluate(
    x: &mut isize,
    cycles: &mut usize,
    prog: &[Command],
    signal_strengts: &mut Vec<(usize, isize)>,
) {
    for cmd in prog {
        match cmd {
            Command::Noop(n) => {
                *cycles += n;
            }
            Command::Addx(i) => {
                *cycles += 2;
                *x += i;
                signal_strengts.push((*cycles + 1, *x));
            }
        }
    }
}

fn run_1(input: &str) -> anyhow::Result<isize> {
    let (_, prog) = parse(input).map_err(|e| anyhow::anyhow!("{e}"))?;
    let mut signal_strengts = vec![(0, 1)];
    let mut x = 1;
    let mut cycles = 0;
    evaluate(&mut x, &mut cycles, &prog, &mut signal_strengts);

    let max_cycles = *signal_strengts.iter().map(|(c, _)| c).max().unwrap();
    let last_x = signal_strengts[signal_strengts.len() - 1].1;

    signal_strengts.push((usize::MAX, last_x));

    let mut check_cycle = 20;
    let mut sum = 0;
    while check_cycle < max_cycles {
        let x_pos = signal_strengts
            .iter()
            .position(|(c, _)| check_cycle < *c)
            .unwrap();
        let x = signal_strengts[x_pos - 1].1;
        let ss = x * check_cycle as isize;
        sum += ss;
        check_cycle += 40;
    }
    Ok(sum)
}

fn run_2(input: &str) -> anyhow::Result<String> {
    let (_, prog) = parse(input).map_err(|e| anyhow::anyhow!("{e}"))?;

    let mut new_prog = Vec::new();
    // simplify prog by adding a noop before each addx and expand noops
    for cmd in prog {
        match cmd {
            Command::Noop(x) => {
                for _ in 0..x {
                    new_prog.push(Command::Noop(1));
                }
            }
            Command::Addx(a) => {
                new_prog.push(Command::Noop(1));
                new_prog.push(Command::Addx(a));
            }
        }
    }

    let prog = new_prog;
    let mut sprite_x = 1;

    let mut res = String::with_capacity(6 * 20);

    for i in 0..(6 * 40) {
        if i != 0 && i % 40 == 0 {
            res += "\n";
        }
        let x = (i % 40) as isize;

        if x >= (sprite_x - 1) && x <= (sprite_x + 1) {
            res += "#";
        } else {
            res += ".";
        }

        if let Some(cmd) = prog.get(i) {
            match cmd {
                Command::Noop(_) => {}
                Command::Addx(a) => {
                    sprite_x += a;
                }
            }
        }
    }

    Ok(res)
}

#[derive(Debug, PartialEq)]
enum Command {
    Noop(usize),
    Addx(isize),
}

fn parse(i: crate::Input) -> crate::PResult<Vec<Command>> {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{i64, newline},
        combinator::map,
        multi::separated_list1,
        sequence::preceded,
    };
    let noop = map(separated_list1(newline, tag("noop")), |v: Vec<&str>| {
        Command::Noop(v.len())
    });

    let addx = map(preceded(tag("addx "), i64), |i| Command::Addx(i as isize));

    let cmd = alt((noop, addx));

    let res = separated_list1(newline, cmd)(i)?;

    Ok(res)
}

#[cfg(test)]
mod tests {
    const INPUT_1: &str = "noop
addx 3
addx -5";

    const MULTI_NOOP: &str = "noop
noop
noop
addx 1";

    #[test]
    fn aoc10_parse() {
        let (_, cmds) = super::parse(INPUT_1).unwrap();

        assert_eq!(cmds.len(), 3);

        let (_, cmds) = super::parse(MULTI_NOOP).unwrap();

        assert_eq!(cmds.len(), 2);
        assert_eq!(cmds[0], super::Command::Noop(3));

        super::parse(INPUT_2).unwrap();
    }
    #[test]
    fn aoc10_eval() {
        let (_, prog) = super::parse(INPUT_1).unwrap();
        let mut signal_strengts = vec![(0, 1)];
        let mut x = 1;
        let mut cycles = 0;
        super::evaluate(&mut x, &mut cycles, &prog, &mut signal_strengts);
        assert_eq!(&signal_strengts, &[(0, 1), (4, 4), (6, -1)]);
    }

    #[test]
    fn aoc10_run_1() {
        assert_eq!(super::run_1(INPUT_2).unwrap(), 13140);
    }

    #[test]
    // #[ignore]
    fn aoc10_run_2() {
        assert_eq!(
            super::run_2(INPUT_2).unwrap(),
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        );
    }

    const INPUT_2: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
}
