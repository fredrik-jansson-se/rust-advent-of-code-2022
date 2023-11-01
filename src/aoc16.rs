use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline, u64},
    multi::separated_list1,
};

pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day16.txt").unwrap();
    println!("day16-1: {}", run_1(&input)?);
    println!("day16-2: {}", run_2(&input)?);
    Ok(())
}

fn eval(
    t: usize,
    max_t: usize,
    came_from: &str,
    cur_valve_name: &str,
    valves: &mut HashMap<String, Valve>,
) -> usize {
    if t == max_t {
        return 0;
    }

    let mut res = Vec::with_capacity(10);
    res.push(0);

    for next_valve in &valves.get(cur_valve_name).unwrap().valves.clone() {
        if next_valve != came_from {
            res.push(eval(t + 1, max_t, cur_valve_name, next_valve, valves));
        }
    }

    // Only consider opening this valve if it can produce any flow
    if valves.get(cur_valve_name).unwrap().flow_rate > 0 {
        // Set it to 0 so it's not opened again
        let flow_rate = {
            let cur_valve = valves.get_mut(cur_valve_name).unwrap();
            let flow_rate = cur_valve.flow_rate;
            cur_valve.flow_rate = 0;
            flow_rate
        };
        let flow_addition = (max_t - t) * flow_rate;
        res.push(flow_addition + eval(t + 1, max_t, cur_valve_name, cur_valve_name, valves));
        let cur_valve = valves.get_mut(cur_valve_name).unwrap();
        cur_valve.flow_rate = flow_rate;
    }

    *res.iter().max().unwrap()
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let (_, valves) = parse(input).map_err(|e| anyhow::anyhow!(e.to_string()))?;

    let mut valves: HashMap<String, Valve> =
        valves.into_iter().map(|v| (v.name.clone(), v)).collect();

    Ok(eval(1, 30, "", "AA", &mut valves))
}

fn run_2(input: &str) -> anyhow::Result<usize> {
    let (_, valves) = parse(input).map_err(|e| anyhow::anyhow!(e.to_string()))?;

    let mut valves: HashMap<String, Valve> =
        valves.into_iter().map(|v| (v.name.clone(), v)).collect();

    Ok(eval(1, 26, "", "AA", &mut valves))
}

#[derive(Debug, Clone)]
struct Valve {
    name: String,
    flow_rate: usize,
    valves: Vec<String>,
}

fn parse_valve(i: crate::Input) -> crate::PResult<Valve> {
    let (i, _) = tag("Valve ")(i)?;
    let (i, name) = alpha1(i)?;
    let (i, _) = tag(" has flow rate=")(i)?;
    let (i, flow_rate) = u64(i)?;
    let (i, _) = nom::branch::alt((
        tag("; tunnels lead to valves "),
        tag("; tunnel leads to valve "),
    ))(i)?;
    let (i, valves) = separated_list1(tag(", "), alpha1)(i)?;

    Ok((
        i,
        Valve {
            name: name.to_string(),
            flow_rate: flow_rate as _,
            valves: valves.iter().map(|v| v.to_string()).collect(),
        },
    ))
}
fn parse(i: crate::Input) -> crate::PResult<Vec<Valve>> {
    let res = separated_list1(newline, parse_valve)(i)?;
    Ok(res)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn aoc16_parse() {
        let (_, _valve) =
            super::parse_valve("Valve HH has flow rate=22; tunnel leads to valve GG").unwrap();
        let (_, valves) = super::parse(INPUT).unwrap();
        assert_eq!(valves.len(), 10);
    }

    #[test]
    fn aoc16_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 1651);
    }

    #[test]
    #[ignore]
    fn aoc16_run_2() {
        assert_eq!(super::run_2(INPUT).unwrap(), 1707);
    }
}
