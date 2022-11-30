use std::cell::RefCell;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub fn run() -> anyhow::Result<()> {
    let input = fs::read_to_string("day8.txt")?;
    println!("day8-1: {}", run_1(&input)?);
    println!("day8-2: {}", run_2(&input)?);
    Ok(())
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let io = parse(input)?;

    let outputs: Vec<&Signals> = io.iter().flat_map(|(_, o)| o).collect();

    Ok(outputs
        .iter()
        .filter(|i| [2usize, 3, 4, 7].contains(&i.len()))
        .count())
}

fn analyze(mut input: SignalPatterns, output: &[Signals]) -> usize {
    let true_signals: HashMap<usize, HashSet<char>> = [
        (0, ['a', 'b', 'c', 'e', 'f', 'g'].iter().copied().collect()),
        (1, ['c', 'f'].iter().copied().collect()),
        (2, ['a', 'c', 'd', 'e', 'g'].iter().copied().collect()),
        (3, ['a', 'c', 'd', 'f', 'g'].iter().copied().collect()),
        (4, ['b', 'c', 'd', 'f'].iter().copied().collect()),
        (5, ['a', 'b', 'd', 'f', 'g'].iter().copied().collect()),
        (6, ['a', 'b', 'd', 'e', 'f', 'g'].iter().copied().collect()),
        (7, ['a', 'c', 'f'].iter().copied().collect()),
        (
            8,
            ['a', 'b', 'c', 'd', 'e', 'f', 'g']
                .iter()
                .copied()
                .collect(),
        ),
        (9, ['a', 'b', 'c', 'd', 'f', 'g'].iter().copied().collect()),
    ]
    .iter()
    .cloned()
    .collect();

    let mut signals_in_common = HashMap::new();
    for a in 0..9 {
        for b in (a + 1)..=9 {
            let a_s = true_signals.get(&a).unwrap();
            let b_s = true_signals.get(&b).unwrap();
            signals_in_common.insert((a, b), a_s.intersection(b_s).count());
        }
    }

    let mut possibles = HashMap::new();
    // Find 1, len 2 in input
    let idx = input.iter().position(|i| i.len() == 2).unwrap();
    possibles.insert(1, RefCell::new(vec![input.remove(idx)]));

    // Find 4, len 4 in input
    let idx = input.iter().position(|i| i.len() == 4).unwrap();
    possibles.insert(4, RefCell::new(vec![input.remove(idx)]));

    // Find 7, len 3 in input
    let idx = input.iter().position(|i| i.len() == 3).unwrap();
    possibles.insert(7, RefCell::new(vec![input.remove(idx)]));

    // Find 8, len 7 in input
    let idx = input.iter().position(|i| i.len() == 7).unwrap();
    possibles.insert(8, RefCell::new(vec![input.remove(idx)]));

    for i in 0..=9 {
        possibles
            .entry(i)
            .or_insert_with(|| RefCell::new(input.clone()));
    }

    while possibles.iter().any(|(_k, v)| v.borrow().len() > 1) {
        for i in 0..9 {
            for j in (i + 1)..=9 {
                let mut iv = possibles.get(&i).unwrap().borrow_mut();
                let mut jv = possibles.get(&j).unwrap().borrow_mut();
                let in_common = signals_in_common.get(&(i, j)).unwrap();
                iv.retain(|v1| {
                    jv.iter()
                        .any(|v2| v1.intersection(v2).count() == *in_common)
                });
                jv.retain(|v1| {
                    iv.iter()
                        .any(|v2| v1.intersection(v2).count() == *in_common)
                });
            }
        }
    }

    let mut res = 0;
    for o in output.iter() {
        let (k, _) = possibles.iter().find(|(_, v)| &v.borrow()[0] == o).unwrap();
        res = res * 10 + k;
    }
    res
}

fn run_2(input: &str) -> anyhow::Result<usize> {
    let io = parse(input)?;

    Ok(io.into_iter().map(|(i, o)| analyze(i, &o)).sum())
}

fn signal_pattern(i: &str) -> nom::IResult<&str, Signals> {
    let (i, s) = nom::character::complete::alpha1(i)?;

    let signals = s.chars().collect();
    Ok((i, signals))
}

fn input_output(i: &str) -> nom::IResult<&str, (SignalPatterns, OutputValues)> {
    let signals =
        |i| nom::multi::separated_list1(nom::bytes::complete::tag(" "), signal_pattern)(i);
    nom::sequence::separated_pair(signals, nom::bytes::complete::tag(" | "), signals)(i)
}

type Signals = std::collections::HashSet<char>;
type SignalPatterns = Vec<Signals>;
type OutputValues = Vec<Signals>;
fn parse(i: &str) -> anyhow::Result<Vec<(SignalPatterns, OutputValues)>> {
    let (_, res) = nom::multi::separated_list1(nom::character::complete::newline, input_output)(i)
        .map_err(|e| e.to_owned())?;

    Ok(res)
}

#[cfg(test)]
mod tests {
    const INPUT: &str =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

    const INPUT_2: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn aoc8_parse() {
        let rows = super::parse(INPUT).unwrap();
        let (patterns, output) = &rows[0];
        assert_eq!(patterns.len(), 10);
        assert_eq!(output.len(), 4);
    }

    #[test]
    fn aoc8_run_1() {
        assert_eq!(super::run_1(INPUT_2).unwrap(), 26);
    }

    #[test]
    fn aoc8_run_2() {
        assert_eq!(super::run_2(INPUT).unwrap(), 5353);
        assert_eq!(super::run_2(INPUT_2).unwrap(), 61229);
    }
}
