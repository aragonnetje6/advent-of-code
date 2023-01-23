use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::{alpha1, char, digit1, newline};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::IResult;

#[derive(Debug, Eq, PartialEq, Clone)]
enum FsObject {
    Dir { name: String, items: Vec<FsObject> },
    File { name: String, size: u64 },
}

impl FsObject {
    fn get_size(&self) -> u64 {
        match self {
            Self::Dir { items, .. } => items.iter().map(Self::get_size).sum(),
            Self::File { size, .. } => *size,
        }
    }

    fn add(&mut self, path: &[&str], entries: &Vec<FsObject>) -> Result<(), String> {
        match self {
            Self::Dir { name, items } => {
                if path.first().unwrap() != name {
                    Err("Invalid path".to_string())
                } else if path.len() == 1 {
                    items.extend(entries.clone());
                    Ok(())
                } else if let Some(item) = items
                    .iter_mut()
                    .find(|x| &x.get_name() == path.get(1).unwrap())
                {
                    item.add(&path[1..], entries)
                } else {
                    Err(format!(
                        "{} Not found in {}",
                        self.get_name(),
                        path.first().unwrap()
                    ))
                }
            }
            Self::File { .. } => Err("Not a dir".to_string()),
        }
    }

    fn list_dirs(&self) -> Vec<&Self> {
        match self {
            Self::File { .. } => vec![],
            Self::Dir { items, .. } => {
                let mut result: Vec<&FsObject> = items.iter().flat_map(Self::list_dirs).collect();
                result.push(self);
                result
            }
        }
    }

    fn get_name(&self) -> &str {
        match self {
            Self::File { name, .. } | Self::Dir { name, .. } => name,
        }
    }
}

#[derive(Debug)]
enum Command {
    CD(String),
    LS(Vec<FsObject>),
}

fn cd(i: &str) -> IResult<&str, Command> {
    let (i, _) = tag("cd ")(i)?;
    let (i, name) = alt((tag(".."), tag("/"), alpha1))(i)?;
    Ok((i, Command::CD(name.to_string())))
}

fn filename_with_extension(i: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_alphabetic() || c == '.')(i)
}

fn file(i: &str) -> IResult<&str, FsObject> {
    let (i, size) = map_res(digit1, str::parse)(i)?;
    let (i, _) = char(' ')(i)?;
    let (i, name) = alt((filename_with_extension, alpha1))(i)?;
    Ok((
        i,
        FsObject::File {
            name: name.to_string(),
            size,
        },
    ))
}

fn dir(i: &str) -> IResult<&str, FsObject> {
    let (i, _) = tag("dir ")(i)?;
    let (i, name) = alpha1(i)?;
    Ok((
        i,
        FsObject::Dir {
            name: name.to_string(),
            items: vec![],
        },
    ))
}

fn entry(i: &str) -> IResult<&str, FsObject> {
    alt((dir, file))(i)
}

fn ls(i: &str) -> IResult<&str, Command> {
    let (i, _) = tag("ls")(i)?;
    let (i, _) = newline(i)?;
    let (i, entries) = separated_list1(newline, entry)(i)?;
    Ok((i, Command::LS(entries)))
}

fn command(i: &str) -> IResult<&str, Command> {
    let (i, _) = tag("$ ")(i)?;
    alt((cd, ls))(i)
}

fn commands(i: &str) -> IResult<&str, Vec<Command>> {
    separated_list1(newline, command)(i)
}

fn get_file_tree(commands: &[Command]) -> FsObject {
    let mut tree = FsObject::Dir {
        name: "/".to_string(),
        items: vec![],
    };
    let mut current_node_stack = vec!["/"];
    for command in commands {
        match command {
            Command::CD(name) => match name.as_str() {
                ".." => {
                    current_node_stack.pop();
                }
                "/" => current_node_stack = vec!["/"],
                name => {
                    current_node_stack.push(name);
                }
            },
            Command::LS(entries) => tree.add(&current_node_stack, entries).unwrap(),
        }
    }
    tree
}

pub fn part1(input: &str) -> String {
    let (_, data) = commands(input).unwrap();
    let tree = get_file_tree(&data);

    tree.list_dirs()
        .iter()
        .map(|x| x.get_size())
        .filter(|x| *x <= 100_000)
        .sum::<u64>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let (_, data) = commands(input).unwrap();
    let tree = get_file_tree(&data);
    let must_clean = 30_000_000 - (70_000_000 - tree.get_size());

    tree.list_dirs()
        .iter()
        .map(|x| x.get_size())
        .filter(|x| *x >= must_clean)
        .min()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA1: &str = r"$ cd /
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
    fn test_part1() {
        assert_eq!(part1(DATA1), "95437");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA1), "24933642");
    }
}
