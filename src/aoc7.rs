use std::fs;

pub fn run() -> anyhow::Result<()> {
    let input = fs::read_to_string("day7.txt")?;
    println!("day7-1: {}", run_1(&input)?);
    println!("day7-2: {}", run_2(&input)?);
    Ok(())
}

pub fn run_1(input: &str) -> anyhow::Result<usize> {
    let (_, listing) = parse(input).map_err(|e| anyhow::anyhow!("{e}"))?;

    dbg! {&listing};

    fn calc_sizes(
        p: &std::path::Path,
        lookup: &mut std::collections::HashMap<std::path::PathBuf, (usize, usize)>,
        listing: &DirListing,
    ) -> usize {
        let dir_items = listing.get(p).unwrap();
        let _lu = lookup.entry(p.to_owned()).or_insert((0, 0));
        // dbg! {di};
        for _item in dir_items {
            // match item {
            //     // &DirItem::Dir(d) {
            //     //     lu.1 += calc_sizes(d);
            //     // }

            // }
        }
        0
    }

    let mut lookup = std::collections::HashMap::new();
    let start: std::path::PathBuf = "/".parse()?;
    let res = calc_sizes(&start, &mut lookup, &listing);

    Ok(res)
}

pub fn run_2(_input: &str) -> anyhow::Result<isize> {
    Ok(0)
}

#[derive(Debug, PartialEq)]
enum Command {
    Ls,
    CdRoot,
    CdParent,
    Cd(String),
}

fn parse_command(i: crate::Input) -> crate::PResult<Command> {
    use nom::{branch::alt, bytes::complete::tag, combinator::map};
    let (i, _) = tag("$ ")(i)?;
    let ls = map(tag("ls"), |_| Command::Ls);
    let root = map(tag("/"), |_| Command::CdRoot);
    let parent = map(tag(".."), |_| Command::CdParent);
    let dir = map(nom::character::complete::alphanumeric1, |dir_name: &str| {
        Command::Cd(dir_name.to_string())
    });
    let cd = nom::sequence::preceded(nom::bytes::complete::tag("cd "), alt((root, parent, dir)));

    let res = nom::sequence::terminated(alt((ls, cd)), nom::character::complete::newline)(i)?;
    Ok(res)
}

#[derive(Debug, PartialEq)]
enum DirItem {
    File(String, usize),
    Dir(String),
}

struct File {
    size: usize,
    name: String,
}

fn parse_filename(i: crate::Input) -> crate::PResult<String> {
    let (i, first) = nom::character::complete::alphanumeric1(i)?;
    let (i, last) = nom::combinator::opt(nom::sequence::pair(
        nom::bytes::complete::tag("."),
        nom::character::complete::alphanumeric0,
    ))(i)?;

    let filename = first.to_string()
        + &last
            .map(|(s1, s2)| s1.to_owned() + s2)
            .unwrap_or_else(String::new);

    Ok((i, filename))
}

fn parse_dir_item(i: crate::Input) -> crate::PResult<DirItem> {
    use nom::{branch::alt, bytes::complete::tag, combinator::map};
    let dir = map(
        nom::sequence::preceded(tag("dir "), nom::character::complete::alphanumeric0),
        |dir_name: &str| DirItem::Dir(dir_name.to_string()),
    );
    let file = map(
        nom::sequence::separated_pair(nom::character::complete::u64, tag(" "), parse_filename),
        |(size, file_name): (u64, String)| DirItem::File(file_name, size as usize),
    );

    let res = alt((dir, file))(i)?;

    Ok(res)
}

type DirListing = std::collections::HashMap<std::path::PathBuf, Vec<DirItem>>;

fn parse(mut i: crate::Input) -> crate::PResult<DirListing> {
    let mut cur_dir: std::path::PathBuf = "/".parse().unwrap();
    let mut listing = DirListing::new();
    while !i.is_empty() {
        // dbg! {i};
        let (ii, cmd) = parse_command(i)?;
        // dbg! {&cmd};

        i = match cmd {
            Command::CdRoot => {
                cur_dir = "/".parse().unwrap();
                ii
            }
            Command::CdParent => {
                cur_dir = cur_dir.parent().unwrap().to_owned();
                ii
            }
            Command::Cd(dir) => {
                cur_dir = cur_dir.join(dir);
                ii
            }
            Command::Ls => {
                let (ii, mut ls) = nom::sequence::terminated(
                    nom::multi::separated_list0(nom::character::complete::newline, parse_dir_item),
                    nom::combinator::opt(nom::character::complete::newline),
                )(ii)?;

                let entries = listing.entry(cur_dir.clone()).or_insert_with(Vec::new);
                entries.append(&mut ls);
                // dbg! {ls};
                ii
            }
        };
    }
    Ok((i, listing))
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn aoc7_parse() {
        let (_, _pos) = super::parse(INPUT).unwrap();
    }

    #[test]
    fn aoc7_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 37);
    }

    #[test]
    fn aoc7_run_2() {}
}
