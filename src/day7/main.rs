use gcollections::ops::Cardinality;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_till1};
use nom::character::complete::{digit1, newline};
use nom::character::is_newline;
use nom::combinator::eof;
use nom::multi::many1;
use nom::sequence::terminated;
use nom::IResult;
use std::collections::HashSet;

/*
 * Tree like structure
 */
type NodeId = usize;

struct Arena {
    nodes: Vec<Node>,
}

impl Arena {
    fn cd(&self, cwd: NodeId, name: &str) -> Option<NodeId> {
        let cwd_dir = match self.nodes.get(cwd) {
            Some(Node::Dir(dir)) => dir,
            _ => return None,
        };

        match name {
            "/" => Some(0),
            ".." => cwd_dir.parent,
            _ => {
                for child in &cwd_dir.childrens {
                    if name == self.nodes[*child].name() {
                        return Some(*child);
                    }
                }
                None
            }
        }
    }

    fn has_child(&self, cwd: NodeId, child: &Node) -> bool {
        match &self.nodes[cwd] {
            Node::Dir(dir) => dir.childrens.iter().any(|&c| &self.nodes[c] == child),
            _ => false,
        }
    }

    fn insert_child(&mut self, cwd: NodeId, mut child: Node) -> Option<NodeId> {
        match child {
            Node::Dir(ref mut dir) => dir.parent = Some(cwd),
            _ => {}
        }
        self.nodes.push(child);
        let new_id = self.nodes.len() - 1;

        match self.nodes.get_mut(cwd) {
            Some(Node::Dir(dir)) => {
                dir.childrens.insert(new_id);
                Some(new_id)
            }
            _ => {
                self.nodes.pop();
                None
            }
        }
    }

    fn get_size(&self, node: NodeId) -> u64 {
        match &self.nodes[node] {
            Node::File(file) => file.size,
            Node::Dir(dir) => dir.childrens.iter().map(|&c| self.get_size(c)).sum(),
        }
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
struct Dir {
    name: String,
    childrens: HashSet<NodeId>,
    parent: Option<NodeId>,
}

#[derive(Debug, PartialEq, Eq)]
struct File {
    name: String,
    size: u64,
}

#[derive(Debug, PartialEq, Eq)]
enum Node {
    Dir(Dir),
    File(File),
}

trait Nodable {
    fn size(&self) -> u64;
    fn name(&self) -> &str;
}

impl Nodable for Dir {
    fn size(&self) -> u64 {
        0
    }
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl Nodable for File {
    fn size(&self) -> u64 {
        self.size
    }
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

// uh, that should be simpler
impl Nodable for Node {
    fn size(&self) -> u64 {
        match self {
            Node::Dir(dir) => dir.size(),
            Node::File(file) => file.size(),
        }
    }
    fn name(&self) -> &str {
        match self {
            Node::Dir(dir) => dir.name(),
            Node::File(file) => file.name(),
        }
    }
}

/*
 * Parsing
 */
#[derive(Debug, PartialEq)]
enum Command {
    Cd(String),
    Ls(Vec<Node>),
}

fn parse_ls_line_dir(input: &[u8]) -> IResult<&[u8], Node> {
    let (input, _) = tag("dir ")(input)?;
    let (input, name) = take_till1(is_newline)(input)?;
    Ok((
        input,
        Node::Dir(Dir {
            name: String::from_utf8(name.to_vec()).unwrap(),
            childrens: HashSet::new(),
            ..Default::default()
        }),
    ))
}
fn parse_ls_line_file(input: &[u8]) -> IResult<&[u8], Node> {
    let (input, size) = digit1(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, name) = take_till1(is_newline)(input)?;
    Ok((
        input,
        Node::File(File {
            name: String::from_utf8(name.to_vec()).unwrap(),
            size: String::from_utf8(size.to_vec()).unwrap().parse().unwrap(),
        }),
    ))
}
fn parse_ls_line(input: &[u8]) -> IResult<&[u8], Node> {
    terminated(alt((parse_ls_line_dir, parse_ls_line_file)), newline)(input)
}
fn parse_ls(input: &[u8]) -> IResult<&[u8], Command> {
    let (input, _) = tag("$ ls\n")(input)?;
    let (input, lines) = many1(parse_ls_line)(input)?;
    Ok((input, Command::Ls(lines)))
}
fn parse_cd(input: &[u8]) -> IResult<&[u8], Command> {
    let (input, _) = tag("$ cd ")(input)?;
    let (input, name) = terminated(take_till1(is_newline), newline)(input)?;
    Ok((
        input,
        Command::Cd(String::from_utf8(name.to_vec()).unwrap()),
    ))
}
fn parse_input(input: &[u8]) -> IResult<&[u8], Vec<Command>> {
    let (input, commands) = many1(alt((parse_ls, parse_cd)))(input)?;
    eof(input)?;
    Ok((input, commands))
}

/*
 * Lets go
 */

fn main() {
    let mut input = std::fs::read("./src/day7/input.txt").unwrap();
    input.push(b'\n'); // add a newline to make sure the last command is parsed
    let (_, commands) = parse_input(&input).unwrap();

    let mut arena = Arena { nodes: Vec::new() };
    arena.nodes.push(Node::Dir(Dir::default()));
    let mut cwd = 0;

    for command in commands {
        match command {
            Command::Cd(name) => {
                // println!("cd {}", name);
                match arena.cd(cwd, name.as_str()) {
                    Some(d) => cwd = d,
                    None => {
                        panic!("cd: {}: No such file or directory", name);
                    }
                }
            }
            Command::Ls(nodes) => {
                // println!("ls");
                for node in nodes {
                    if !arena.has_child(cwd, &node) {
                        arena.insert_child(cwd, node).unwrap();
                    }
                }
            }
        }
    }
    let mut total_sum = 0;
    for i in 0..arena.nodes.len() {
        match &arena.nodes[i] {
            Node::Dir(dir) => {
                let dir_size = arena.get_size(i);
                if dir_size <= 100000 {
                    total_sum += dir_size;
                }
            }
            _ => {}
        }
    }
    println!("Puzzle 1: {}", total_sum);

    const FS_SIZE: u64 = 70000000;
    const SIZE_NEEDED: u64 = 30000000;
    let unused_space = FS_SIZE - arena.get_size(0);
    let must_free_min = SIZE_NEEDED - unused_space;

    let mut current_min_dir_size_to_delete = arena.get_size(0);
    for i in 0..arena.nodes.len() {
        match &arena.nodes[i] {
            Node::Dir(dir) => {
                let dir_size = arena.get_size(i);
                if dir_size >= must_free_min && dir_size < current_min_dir_size_to_delete {
                    current_min_dir_size_to_delete = dir_size;
                }
            }
            _ => {}
        }
    }
    println!("Puzzle 2: {}", current_min_dir_size_to_delete);
}

#[cfg(test)]
mod tests {
    use crate::{
        parse_input, parse_ls, parse_ls_line_dir, parse_ls_line_file, Command, Dir, File, Node,
        Size,
    };
    use std::collections::HashSet;

    #[test]
    fn test_parse_ls_line_dir() {
        let input = b"dir /home/user\n";
        let expected = Node::Dir(Dir {
            name: String::from("/home/user"),
            childrens: HashSet::new(),
            ..Default::default()
        });
        assert_eq!(parse_ls_line_dir(input), Ok((&b"\n"[..], expected)));
    }

    #[test]
    fn test_parse_ls_line_file() {
        let input = b"1234 file.txt\n";
        let expected = Node::File(File {
            name: String::from("file.txt"),
            size: 1234,
        });
        assert_eq!(parse_ls_line_file(input), Ok((&b"\n"[..], expected)));
    }

    #[test]
    fn test_parse_ls() {
        let input = b"$ ls
dir e
62596 h.lst
dir z
";
        let expected = Command::Ls(vec![
            Node::Dir(Dir {
                name: String::from("e"),
                childrens: HashSet::new(),
                ..Default::default()
            }),
            Node::File(File {
                name: String::from("h.lst"),
                size: 62596,
            }),
            Node::Dir(Dir {
                name: String::from("z"),
                childrens: HashSet::new(),
                ..Default::default()
            }),
        ]);
        assert_eq!(parse_ls(input), Ok((&b""[..], expected)));
    }

    #[test]
    fn test_parse_input() {
        let input = b"$ cd /
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
";
        let expected = vec![
            Command::Cd(String::from("/")),
            Command::Ls(vec![
                Node::Dir(Dir {
                    name: String::from("a"),
                    childrens: HashSet::new(),
                    ..Default::default()
                }),
                Node::File(File {
                    name: String::from("b.txt"),
                    size: 14848514,
                }),
                Node::File(File {
                    name: String::from("c.dat"),
                    size: 8504156,
                }),
                Node::Dir(Dir {
                    name: String::from("d"),
                    childrens: HashSet::new(),
                    ..Default::default()
                }),
            ]),
            Command::Cd(String::from("a")),
            Command::Ls(vec![
                Node::Dir(Dir {
                    name: String::from("e"),
                    childrens: HashSet::new(),
                    ..Default::default()
                }),
                Node::File(File {
                    name: String::from("f"),
                    size: 29116,
                }),
                Node::File(File {
                    name: String::from("g"),
                    size: 2557,
                }),
                Node::File(File {
                    name: String::from("h.lst"),
                    size: 62596,
                }),
            ]),
            Command::Cd(String::from("e")),
        ];
        let (_, commands) = parse_input(input).unwrap();
        assert_eq!(commands, expected);
    }
}
