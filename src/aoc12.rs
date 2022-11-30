use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub fn run() -> anyhow::Result<()> {
    let input = fs::read_to_string("day12.txt").unwrap();
    println!("day12-1: {}", run_1(&input)?);
    println!("day12-2: {}", run_2(&input)?);
    Ok(())
}

type Map<'a> = HashMap<&'a str, Vec<&'a str>>;
type ToVisit<'a> = Vec<&'a str>;

fn get_nbrs<'a>(current: &'a str, map: &Map<'a>, visited: &HashSet<&'a str>) -> Vec<&'a str> {
    map.get(current)
        .expect("find current")
        .iter()
        .filter(|v| !visited.contains(*v))
        .copied()
        .collect()
}

fn get_nbrs2<'a>(current: &'a str, map: &Map<'a>, to_visit: &[&'a str]) -> Vec<&'a str> {
    map.get(current)
        .expect("find current")
        .iter()
        .filter(|v| to_visit.contains(v))
        .copied()
        .collect()
}

fn search<'a>(current: &'a str, map: &Map<'a>, mut visited: HashSet<&'a str>) -> Vec<Vec<&'a str>> {
    if current == "end" {
        return vec![vec!["end"]];
    }

    let mut res = Vec::new();
    if current.chars().any(|c| c.is_lowercase()) {
        visited.insert(current);
    }
    for n in get_nbrs(current, map, &visited) {
        let mut paths = search(n, map, visited.clone());
        for p in paths.iter_mut() {
            p.push(current);
        }
        res.append(&mut paths);
    }
    res
}

fn search2<'a>(current: &'a str, map: &Map<'a>, mut to_visit: ToVisit<'a>) -> Vec<Vec<&'a str>> {
    if current == "end" {
        return vec![vec!["end"]];
    }

    let mut res = Vec::new();
    if current.chars().any(|c| c.is_lowercase()) {
        if let Some(pos) = to_visit.iter().position(|c| *c == current) {
            to_visit.remove(pos);
        }
    }

    for n in get_nbrs2(current, map, &to_visit) {
        let mut paths = search2(n, map, to_visit.clone());
        for p in paths.iter_mut() {
            p.push(current);
        }
        res.append(&mut paths);
    }
    res
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let map = parse(input);
    Ok(search("start", &map, HashSet::new()).len())
}

fn run_2(input: &str) -> anyhow::Result<usize> {
    let map = parse(input);

    let visit_twice: Vec<&str> = map
        .keys()
        .filter(|c| **c != "start" && **c != "end")
        .filter(|c| c.chars().any(|c| c.is_lowercase()))
        .copied()
        .collect();

    let mut paths = HashSet::new();

    for vt in visit_twice {
        let mut to_visit: Vec<&str> = map.keys().copied().collect();
        to_visit.push(vt);
        for p in search2("start", &map, to_visit) {
            paths.insert(p);
        }
    }

    Ok(paths.len())
}

fn parse(input: &str) -> Map {
    let mut res: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let mut s = line.split('-');
        let a = s.next().expect("key");
        let b = s.next().expect("value");
        res.entry(a).or_default().push(b);
        res.entry(b).or_default().push(a);
    }

    res
}

#[cfg(test)]
mod tests {
    const INPUT_1: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    const INPUT_2: &str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

    const INPUT_3: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    #[test]
    fn aoc12_run_1() {
        assert_eq!(super::run_1(INPUT_1).unwrap(), 10);
        assert_eq!(super::run_1(INPUT_2).unwrap(), 19);
        assert_eq!(super::run_1(INPUT_3).unwrap(), 226);
    }

    #[test]
    fn aoc12_run_2() {
        assert_eq!(super::run_2(INPUT_1).unwrap(), 36);
        assert_eq!(super::run_2(INPUT_2).unwrap(), 103);
        assert_eq!(super::run_2(INPUT_3).unwrap(), 3509);
    }
}
