use std::{collections::HashMap, fs};

pub fn run() -> anyhow::Result<()> {
    let input = fs::read_to_string("day14.txt").unwrap();
    println!("day14-1: {}", run_1(&input)?);
    println!("day14-2: {}", run_2(&input)?);
    Ok(())
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let (_, (mut template, recipies)) = parse(input).unwrap();

    for _ in 0..10 {
        let mut new_template = Vec::with_capacity(2 * template.len());

        for ab in template.windows(2) {
            let a = ab[0];
            let b = ab[1];
            new_template.push(a);
            new_template.push(*recipies.get(&(a, b)).unwrap());
        }
        // last char not covered by the windows fn, add it back
        new_template.push(template[template.len() - 1]);

        template = new_template;
    }

    let mut counts = HashMap::new();
    for c in template {
        *counts.entry(c).or_insert(0usize) += 1;
    }

    let (min, max) = counts
        .values()
        .fold((usize::MAX, 0), |(min, max), v| (min.min(*v), max.max(*v)));

    Ok(max - min)
}

fn run_2(input: &str) -> anyhow::Result<usize> {
    let (_, (mut template, recipies)) = parse(input).unwrap();

    for i in 0..40 {
        let mut new_template = Vec::with_capacity(2 * template.len());

        for ab in template.windows(2) {
            let a = ab[0];
            let b = ab[1];
            new_template.push(a);
            new_template.push(*recipies.get(&(a, b)).unwrap());
        }
        // last char not covered by the windows fn, add it back
        new_template.push(template[template.len() - 1]);

        template = new_template;
        dbg! {(i, template.len())};
    }

    let mut counts = HashMap::new();
    for c in template {
        *counts.entry(c).or_insert(0usize) += 1;
    }

    let (min, max) = counts
        .values()
        .fold((usize::MAX, 0), |(min, max), v| (min.min(*v), max.max(*v)));

    Ok(max - min)
}

type Lookup = HashMap<(char, char), char>;
fn parse(i: &str) -> nom::IResult<&str, (Vec<char>, Lookup)> {
    // let template = nom::multi::many1(nom::character::complete::anychar);
    let template = nom::character::complete::alpha1;

    let recipy = nom::sequence::separated_pair(
        nom::sequence::pair(
            nom::character::complete::anychar,
            nom::character::complete::anychar,
        ),
        nom::bytes::complete::tag(" -> "),
        nom::character::complete::anychar,
    );

    let recipies = nom::multi::separated_list1(nom::character::complete::newline, recipy);

    let (i, (template, recipies)) = nom::sequence::separated_pair(
        template,
        nom::multi::many1(nom::character::complete::newline),
        recipies,
    )(i)?;

    Ok((
        i,
        (template.chars().collect(), recipies.into_iter().collect()),
    ))
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
    #[test]
    fn aoc14_parse() {
        let (_, (template, recipies)) = super::parse(INPUT).unwrap();
        assert_eq!(template.len(), 4);
        assert_eq!(recipies.len(), 16);
    }
    #[test]
    fn aoc14_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 1588);
    }
    #[test]
    fn aoc14_run_2() {
        // assert_eq!(super::run_2(INPUT).unwrap(), 2188189693529);
    }
}
