use std::fs;

pub fn run() -> anyhow::Result<()> {
    let input = fs::read_to_string("day7.txt")?;
    println!("day7-1: {}", run_1(&input)?);
    println!("day7-2: {}", run_2(&input)?);
    Ok(())
}

pub fn run_1(input: &str) -> anyhow::Result<usize> {
    let (_, listing) = parse(input).map_err(|e| anyhow::anyhow!("{e}"))?;

    let dir: Directory = listing.try_into()?;

    let mut sizes = Vec::new();

    fn calc(d: &Directory, sizes: &mut Vec<usize>) {
        d.directories.iter().for_each(|d| calc(d, sizes));
        if d.size < 100_000 {
            sizes.push(d.size);
        }
    }

    calc(&dir, &mut sizes);

    Ok(sizes.into_iter().sum())
}

pub fn run_2(input: &str) -> anyhow::Result<usize> {
    let (_, listing) = parse(input).map_err(|e| anyhow::anyhow!("{e}"))?;

    let dir: Directory = listing.try_into()?;
    const TOTAL_SIZE: usize = 70_000_000;
    const NEEDED: usize = 30_000_000;

    let unused = TOTAL_SIZE - dir.size;

    let mut sizes = Vec::new();

    fn calc(d: &Directory, sizes: &mut Vec<usize>) {
        d.directories.iter().for_each(|d| calc(d, sizes));

        sizes.push(d.size);
    }
    calc(&dir, &mut sizes);

    // Would deleting this directory free up enough space?
    sizes.retain(|s| s + unused >= NEEDED);

    Ok(*sizes.iter().min().unwrap())
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

#[derive(Debug)]
struct File {}

#[derive(Debug)]
struct Directory {
    // name: String,
    files: Vec<File>,
    directories: Vec<Directory>,
    size: usize,
}

#[derive(Debug, PartialEq)]
enum DirItem {
    File { size: usize },
    Dir { name: String },
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
        |dir_name: &str| DirItem::Dir {
            name: dir_name.to_string(),
        },
    );
    let file = map(
        nom::sequence::separated_pair(nom::character::complete::u64, tag(" "), parse_filename),
        |(size, _): (u64, String)| DirItem::File {
            size: size as usize,
        },
    );

    let res = alt((dir, file))(i)?;

    Ok(res)
}

type DirListing = std::collections::HashMap<std::path::PathBuf, Vec<DirItem>>;

fn parse(mut i: crate::Input) -> crate::PResult<DirListing> {
    let mut cur_dir: std::path::PathBuf = "/".parse().unwrap();
    let mut listing = DirListing::new();
    while !i.is_empty() {
        let (ii, cmd) = parse_command(i)?;

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

                let entries = listing.entry(cur_dir.clone()).or_default();
                entries.append(&mut ls);
                ii
            }
        };
    }
    Ok((i, listing))
}

fn to_dir(d: &DirListing, cur_dir: &std::path::Path) -> anyhow::Result<Directory> {
    let mut dir = Directory {
        files: Default::default(),
        directories: Default::default(),
        size: 0,
    };

    let cur_dir_entry = d.get(cur_dir).ok_or(anyhow::anyhow!("no entry"))?;

    for entry in cur_dir_entry {
        match entry {
            DirItem::File { size } => {
                dir.files.push(File {
                });
                dir.size += *size;
            }
            DirItem::Dir { name } => {
                let sub_dir = to_dir(d, &cur_dir.join(name))?;
                dir.size += sub_dir.size;
                dir.directories.push(sub_dir);
            }
        }
    }

    Ok(dir)
}

impl TryFrom<DirListing> for Directory {
    type Error = anyhow::Error;

    fn try_from(d: DirListing) -> Result<Self, Self::Error> {
        let path = std::path::Path::new("/").to_owned();
        to_dir(&d, &path)
    }
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
        assert_eq!(super::run_1(INPUT).unwrap(), 95437);
    }

    #[test]
    fn aoc7_run_2() {
        assert_eq!(super::run_2(INPUT).unwrap(), 24933642);
    }
}
