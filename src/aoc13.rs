use std::{cmp::Ordering, fs};

pub fn run() -> anyhow::Result<()> {
    let input = fs::read_to_string("day13.txt").unwrap();
    println!("day13-1: {}", run_1(&input)?);
    println!("day13-2: {}", run_2(&input)?);
    Ok(())
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let (_, pairs) = parse(input).map_err(|e| anyhow::anyhow!("{e}"))?;
    let mut sum = 0;
    for (idx, pair) in pairs.iter().enumerate() {
        if pair.0.partial_cmp(&pair.1) == Some(Ordering::Less) {
            sum += idx + 1;
        }
    }
    Ok(sum)
}

fn run_2(input: &str) -> anyhow::Result<usize> {
    let packets = input.lines().filter_map(|line| {
        if line.is_empty() {
            None
        } else {
            parse_packet(line).map(|p| p.1).ok()
        }
    });
    let mut packets: Vec<Packet> = packets.collect();
    let (_, key_1) = parse_packet("[[2]]").unwrap();
    let (_, key_2) = parse_packet("[[6]]").unwrap();
    packets.push(key_1.clone());
    packets.push(key_2.clone());
    packets.sort();

    let key_1_idx = packets.iter().position(|p1| p1 == &key_1).unwrap() + 1;
    let key_2_idx = packets.iter().position(|p1| p1 == &key_2).unwrap() + 1;
    Ok(key_1_idx * key_2_idx)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Int(u32),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Int(i1), Self::Int(i2)) => Some(i1.cmp(i2)),
            (Self::List(l1), Self::List(l2)) => {
                let lens = l1.len().cmp(&l2.len());

                for (v1, v2) in l1.iter().zip(l2.iter()) {
                    match v1.partial_cmp(v2) {
                        Some(std::cmp::Ordering::Equal) => continue,
                        o => return o,
                    }
                }
                // If we can differ above, compare lens
                Some(lens)
            }
            (p1, Self::Int(p2)) => p1.partial_cmp(&Packet::List(vec![Packet::Int(*p2)])),
            (Self::Int(p1), p2) => Packet::List(vec![Packet::Int(*p1)]).partial_cmp(p2),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn parse_int(i: crate::Input) -> crate::PResult<Packet> {
    nom::combinator::map(nom::character::complete::u32, Packet::Int)(i)
}

fn parse_list(i: crate::Input) -> crate::PResult<Packet> {
    let (i, _) = nom::bytes::complete::tag("[")(i)?;
    let (i, packet) = nom::combinator::map(
        nom::multi::separated_list0(nom::bytes::complete::tag(","), parse_packet),
        Packet::List,
    )(i)?;
    let (i, _) = nom::bytes::complete::tag("]")(i)?;
    Ok((i, packet))
}

fn parse_packet(i: crate::Input) -> crate::PResult<Packet> {
    nom::branch::alt((parse_int, parse_list))(i)
}

fn parse_pair(i: crate::Input) -> crate::PResult<(Packet, Packet)> {
    nom::sequence::separated_pair(
        parse_packet,
        nom::character::complete::newline,
        parse_packet,
    )(i)
}

fn parse(i: crate::Input) -> crate::PResult<Vec<(Packet, Packet)>> {
    let double_newline = nom::multi::many_m_n(2, 2, nom::character::complete::newline);

    let (i, pairs) = nom::multi::separated_list0(double_newline, parse_pair)(i)?;
    Ok((i, pairs))
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn aoc13_parse() {
        let (_i, pairs) = super::parse(INPUT).unwrap();
        assert_eq!(pairs.len(), 8);
    }

    #[test]
    fn aoc13_order() {
        let (p1, p2) = super::parse_pair("[1,1,3,1,1]\n[1,1,5,1,1]").unwrap().1;
        assert_eq!(p1.cmp(&p2), std::cmp::Ordering::Less);

        let (p1, p2) = super::parse_pair("[[1],[2,3,4]]\n[[1],4]").unwrap().1;
        assert_eq!(p1.cmp(&p2), std::cmp::Ordering::Less);

        let (p1, p2) = super::parse_pair("[9]\n[[8,7,6]]").unwrap().1;
        assert_eq!(p1.cmp(&p2), std::cmp::Ordering::Greater);
    }

    #[test]
    fn aoc13_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 13);
    }

    #[test]
    fn aoc13_run_2() {
        assert_eq!(super::run_2(INPUT).unwrap(), 140);
    }
}
