use std::{fs, ops::Range};

pub fn run() -> anyhow::Result<()> {
    let input = fs::read_to_string("day22.txt").unwrap();
    println!("22:1 {}", run_1(&input)?);
    println!("22:2 {}", run_2(&input)?);
    Ok(())
}

type Input<'a> = &'a str;
type PResult<'a, O> = nom::IResult<Input<'a>, O, nom::error::VerboseError<Input<'a>>>;

fn clean_range(min: isize, max: isize, r: &mut Range<isize>) {
    if r.contains(&min) {
        r.start = min;
    }
    if r.contains(&max) {
        r.end = max + 1;
    }
    if r.end < min {
        r.end = min;
        r.start = min;
    }
    if r.start > max {
        r.end = max;
        r.start = max;
    }
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let mut steps = parse(input).unwrap().1;

    // filter out ranges outsize -50..=50
    for step in steps.iter_mut() {
        clean_range(-50, 50, &mut step.x);
        clean_range(-50, 50, &mut step.y);
        clean_range(-50, 50, &mut step.z);
    }

    let mut map = std::collections::HashSet::new();
    for step in steps {
        for x in step.x {
            for y in step.y.clone() {
                for z in step.z.clone() {
                    if step.on {
                        map.insert((x, y, z));
                    } else {
                        map.remove(&(x, y, z));
                    }
                }
            }
        }
    }

    Ok(map.len())
}

fn run_2(input: &str) -> anyhow::Result<usize> {
    let steps = parse(input).unwrap().1;

    let mut map = std::collections::HashSet::new();
    let mut i = 0;
    let len = steps.len();
    for step in steps {
        i += 1;
        println!("{i}/{len}");
        for x in step.x {
            for y in step.y.clone() {
                for z in step.z.clone() {
                    if step.on {
                        map.insert((x, y, z));
                    } else {
                        map.remove(&(x, y, z));
                    }
                }
            }
        }
    }

    Ok(map.len())
}

fn parse_range(i: Input) -> PResult<std::ops::Range<isize>> {
    use nom::{
        bytes::complete::tag, character::complete::i32, combinator::map, sequence::separated_pair,
    };

    map(separated_pair(i32, tag(".."), i32), |(l, h)| {
        std::ops::Range {
            start: l as isize,
            end: h as isize + 1,
        }
    })(i)
}

fn parse_named_range<'a>(name: &str, i: Input<'a>) -> PResult<'a, std::ops::Range<isize>> {
    let (i, _) = nom::bytes::complete::tag(name)(i)?;
    let (i, _) = nom::bytes::complete::tag("=")(i)?;
    parse_range(i)
}

#[derive(Debug, PartialEq)]
struct RebootStep {
    on: bool,
    x: std::ops::Range<isize>,
    y: std::ops::Range<isize>,
    z: std::ops::Range<isize>,
}

fn parse_reboot_step(i: Input) -> PResult<RebootStep> {
    use nom::combinator::map;
    let on = map(nom::bytes::complete::tag("on "), |_| true);
    let off = map(nom::bytes::complete::tag("off "), |_| false);

    let (i, on_off) = nom::branch::alt((on, off))(i)?;
    let (i, x) = parse_named_range("x", i)?;
    let (i, _) = nom::bytes::complete::tag(",")(i)?;
    let (i, y) = parse_named_range("y", i)?;
    let (i, _) = nom::bytes::complete::tag(",")(i)?;
    let (i, z) = parse_named_range("z", i)?;

    Ok((
        i,
        RebootStep {
            on: on_off,
            x,
            y,
            z,
        },
    ))
}

fn parse(i: Input) -> PResult<Vec<RebootStep>> {
    nom::multi::separated_list0(nom::character::complete::newline, parse_reboot_step)(i)
}

#[cfg(test)]
mod tests {
    const INPUT_1: &str = "on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10";

    const INPUT_2: &str = "on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682";

    #[test]

    fn aoc22_parse() {
        assert_eq!(-10..11, super::parse_range("-10..10").unwrap().1);

        assert_eq!(
            super::RebootStep {
                on: true,
                x: 10..13,
                y: 10..13,
                z: 10..13
            },
            super::parse_reboot_step("on x=10..12,y=10..12,z=10..12")
                .unwrap()
                .1
        );
    }

    #[test]
    fn aoc22_run_1() {
        assert_eq!(super::run_1(INPUT_1).unwrap(), 39);
        assert_eq!(super::run_1(INPUT_2).unwrap(), 590784);
    }

    #[test]
    fn aoc22_run_2() {
        // assert_eq!(super::run_2(INPUT_3).unwrap(), 2758514936282235);
    }
}
