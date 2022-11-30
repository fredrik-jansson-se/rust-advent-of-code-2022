use std::fs;

type Input<'a> = (&'a [u8], usize);
type PResult<'a, O> = nom::IResult<Input<'a>, O, nom::error::VerboseError<Input<'a>>>;

pub fn run() -> anyhow::Result<()> {
    let input = fs::read_to_string("day16.txt").unwrap();
    println!("day16-1: {}", run_1(&input)?);
    println!("day16-2: {}", run_2(&input)?);
    Ok(())
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let input = str_to_vec(input);
    let (_, packet) =
        parse_packet((&input, 0)).map_err(|e| anyhow::anyhow!("Failed to parse: {:?}", e))?;

    Ok(vsn_sum(packet))
}

fn vsn_sum(p: Packet) -> usize {
    match p {
        Packet::Literal { version, .. } => version,
        Packet::Operator {
            version, packets, ..
        } => packets.into_iter().map(vsn_sum).sum::<usize>() + version,
    }
}

fn eval(p: &Packet) -> usize {
    match p {
        Packet::Literal { lit, .. } => *lit,
        Packet::Operator {
            r#type, packets, ..
        } => match r#type {
            Type::Sum => packets.iter().map(eval).sum(),
            Type::Prod => packets.iter().map(eval).product(),
            Type::Max => packets.iter().map(eval).max().unwrap_or(0),
            Type::Min => packets.iter().map(eval).min().unwrap_or(0),
            Type::LT => {
                let p1 = eval(&packets[0]);
                let p2 = eval(&packets[1]);
                (p1 < p2) as usize
            }
            Type::GT => {
                let p1 = eval(&packets[0]);
                let p2 = eval(&packets[1]);
                (p1 > p2) as usize
            }
            Type::Eq => {
                let p1 = eval(&packets[0]);
                let p2 = eval(&packets[1]);
                (p1 == p2) as usize
            }
            p => {
                dbg! {p};
                todo!()
            }
        },
    }
}

fn run_2(input: &str) -> anyhow::Result<usize> {
    let input = str_to_vec(input);
    let (_, packet) =
        parse_packet((&input, 0)).map_err(|e| anyhow::anyhow!("Failed to parse: {:?}", e))?;

    // dbg! {&packet};

    Ok(eval(&packet))
}

#[derive(Debug, PartialEq)]
enum Packet {
    Literal {
        version: usize,
        lit: usize,
    },
    Operator {
        r#type: Type,
        version: usize,
        packets: Vec<Packet>,
    },
}

#[derive(Clone, Copy, Debug, derive_try_from_primitive::TryFromPrimitive, PartialEq, Eq)]
#[repr(u8)]
enum Type {
    Sum = 0,
    Prod = 1,
    Min = 2,
    Max = 3,
    Lit = 4,
    GT = 5,
    LT = 6,
    Eq = 7,
}

fn parse_literal(io: Input, version: usize) -> PResult<Packet> {
    let high_value = nom::sequence::preceded(
        nom::bits::complete::tag(1, 1usize),
        nom::bits::complete::take(4usize),
    );

    let mut low_value = nom::sequence::preceded(
        nom::bits::complete::tag(0, 1usize),
        nom::bits::complete::take(4usize),
    );

    let (io, highs): (_, Vec<usize>) = nom::multi::many0(high_value)(io)?;
    let (io, low): (_, usize) = low_value(io)?;

    let mut val = 0;
    for h in highs {
        val = (val << 4) + h;
    }

    val = (val << 4) + low;

    Ok((io, Packet::Literal { version, lit: val }))
}

fn parse_operator(io: Input, version: usize, r#type: Type) -> PResult<Packet> {
    #[derive(Debug)]
    enum LT {
        Bits(usize),
        Packets(usize),
    }

    let total_length_bits = nom::combinator::map(
        nom::sequence::preceded(
            nom::bits::complete::tag(0, 1usize),
            nom::bits::complete::take(15usize),
        ),
        LT::Bits,
    );

    let total_length_packets = nom::combinator::map(
        nom::sequence::preceded(
            nom::bits::complete::tag(1, 1usize),
            nom::bits::complete::take(11usize),
        ),
        LT::Packets,
    );

    let (io, len) = nom::branch::alt((total_length_packets, total_length_bits))(io)?;

    use nom::error::context;
    match len {
        LT::Packets(num_packets) => {
            let (io, packets) = context(
                "packets",
                nom::multi::many_m_n(num_packets, num_packets, parse_packet),
            )(io)?;

            // println!("Read {} packets", num_packets);

            // After we're done with n packets, we stop
            // let io: (&[u8], usize) = (&[], 0);
            Ok((
                io,
                Packet::Operator {
                    version,
                    r#type,
                    packets,
                },
            ))
        }

        LT::Bits(bits) => {
            let (sub_packets, io) = split(io, bits);

            let (_, packets) = nom::multi::many1(parse_packet)(sub_packets)?;

            Ok((
                io,
                Packet::Operator {
                    version,
                    r#type,
                    packets,
                },
            ))
        }
    }
}

fn parse_packet(io: Input) -> PResult<Packet> {
    let (io, version): (_, usize) = nom::bits::complete::take(3usize)(io)?;
    let (io, typ): (_, u8) = nom::bits::complete::take(3usize)(io)?;

    let r#type: Type = typ.try_into().unwrap();

    match r#type {
        Type::Lit => {
            let (io, packet) = parse_literal(io, version)?;
            // println!("<== parse_packet");
            Ok((io, packet))
        }
        _ => {
            let (io, packet) = parse_operator(io, version, r#type)?;
            // println!("<== parse_packet");
            Ok((io, packet))
        }
    }
}

fn split(i: Input, mut cnt_bits: usize) -> (Input, Input) {
    let data = i.0;
    let mut bit_offset = i.1;
    let mut sub_end_offset = 0;
    let mut new_offset = 0;
    while cnt_bits > 0 {
        let left_in_byte = 8 - bit_offset;
        // dbg! {(cnt_bits, bit_offset, sub_end_offset)};
        if cnt_bits >= left_in_byte {
            sub_end_offset += 1;
            new_offset += 1;
            bit_offset = 0;
            cnt_bits -= left_in_byte;
        } else {
            sub_end_offset += 1;
            bit_offset = cnt_bits;
            cnt_bits = 0
        }
        // dbg! {(cnt_bits, bit_offset, sub_end_offset)};
    }
    (
        (&data[..sub_end_offset], i.1),
        (&data[new_offset..], bit_offset),
    )
}

fn str_to_vec(i: &str) -> Vec<u8> {
    let mut hex_bytes = i
        .as_bytes()
        .iter()
        .filter_map(|b| match b {
            b'0'..=b'9' => Some(b - b'0'),
            b'a'..=b'f' => Some(b - b'a' + 10),
            b'A'..=b'F' => Some(b - b'A' + 10),
            _ => None,
        })
        .fuse();

    let mut bytes = Vec::new();
    while let (Some(h), Some(l)) = (hex_bytes.next(), hex_bytes.next()) {
        bytes.push(h << 4 | l)
    }
    bytes
}

#[cfg(test)]
mod tests {
    #[test]
    fn aoc16_parse_1() {
        let input = super::str_to_vec("D2FE28");
        assert_eq!(&input, &[0xd2, 0xfe, 0x28]);
        let ((io, _), res) = super::parse_packet((&input, 0)).unwrap();
        assert_eq!(io.len(), 1);
        assert_eq!(
            res,
            super::Packet::Literal {
                version: 6,
                lit: 2021
            }
        );
    }

    #[test]
    fn aoc16_parse_2() {
        let input = super::str_to_vec("38006F45291200");
        let (_, res) = super::parse_packet((&input, 0)).unwrap();
        assert_eq!(
            res,
            super::Packet::Operator {
                version: 1,
                r#type: super::Type::LT,
                packets: vec![
                    super::Packet::Literal {
                        version: 6,
                        lit: 10
                    },
                    super::Packet::Literal {
                        version: 2,
                        lit: 20
                    }
                ],
            }
        );

        let input = super::str_to_vec("EE00D40C823060");
        let (_, res) = super::parse_packet((&input, 0)).unwrap();
        assert_eq!(
            res,
            super::Packet::Operator {
                version: 7,
                r#type: super::Type::Max,
                packets: vec![
                    super::Packet::Literal { version: 2, lit: 1 },
                    super::Packet::Literal { version: 4, lit: 2 },
                    super::Packet::Literal { version: 1, lit: 3 },
                ],
            }
        );
    }

    #[test]
    fn aoc16_run_1() {
        assert_eq!(super::run_1("8A004A801A8002F478").unwrap(), 16);
        assert_eq!(super::run_1("620080001611562C8802118E34").unwrap(), 12);
        assert_eq!(super::run_1("C0015000016115A2E0802F182340").unwrap(), 23);
        assert_eq!(super::run_1("A0016C880162017C3686B18A3D4780").unwrap(), 31);
    }

    #[test]
    fn aoc16_run_2() {
        assert_eq!(super::run_2("C200B40A82").unwrap(), 3);
        assert_eq!(super::run_2("04005AC33890").unwrap(), 54);
        assert_eq!(super::run_2("CE00C43D881120").unwrap(), 9);
        assert_eq!(super::run_2("D8005AC2A8F0").unwrap(), 1);
        assert_eq!(super::run_2("F600BC2D8F").unwrap(), 0);
        assert_eq!(super::run_2("9C005AC2F8F0").unwrap(), 0);
        assert_eq!(super::run_2("9C0141080250320F1802104A08").unwrap(), 1);
    }

    #[test]
    fn aoc16_split() {
        let data = &[0x0fu8, 0xf0];
        let i = (&data[..], 4);
        let (r1, r2) = super::split(i, 8);
        assert_eq!(r1, (&data[..], 4));
        assert_eq!(r2, (&data[1..], 4));

        let data = &[0x0fu8, 0xf0, 0xf0];
        let i = (&data[..], 4);
        let (r1, r2) = super::split(i, 13);
        assert_eq!(r1, (&data[..], 4));
        assert_eq!(r2, (&data[2..], 1));

        let data = &[0x0fu8, 0xf0, 0xf0];
        let i = (&data[..], 4);
        let (r1, r2) = super::split(i, 4);
        assert_eq!(r1, (&data[0..1], 4));
        assert_eq!(r2, (&data[1..], 0));
    }
}
