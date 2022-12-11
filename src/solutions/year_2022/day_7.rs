use crate::HashMap;

pub fn part_one(input: &str) -> usize {
    let dir_size = parse_directory_sizes(input);
    let mut total_size = 0;

    for path in dir_size.keys() {
        let size: usize = dir_size
            .iter()
            .filter_map(|(k, v)| k.starts_with(path).then_some(*v))
            .sum();
        if size <= 100000 {
            total_size += size;
        }
    }

    total_size
}

pub fn part_two(input: &str) -> usize {
    let dir_size = parse_directory_sizes(input);
    let total_space = 70000000;
    let needed_space = 30000000;
    let used_space: usize = dir_size.values().copied().sum::<usize>();
    let free_space = total_space - used_space;

    let mut best_size = usize::MAX;

    for path in dir_size.keys() {
        let size: usize = dir_size
            .iter()
            .filter_map(|(k, v)| k.starts_with(path).then_some(*v))
            .sum();
        if free_space + size >= needed_space {
            best_size = size.min(best_size);
        }
    }

    best_size
}

fn parse_directory_sizes(input: &str) -> HashMap<String, usize> {
    let mut dir_size = HashMap::new();
    let commands: Vec<_> = input
        .split("$")
        .filter_map(|c| c.parse::<Command>().ok())
        .collect();
    let mut path_stack = Vec::new();

    for command in commands.iter() {
        match command {
            Command::CdParent => {
                path_stack.pop();
            }
            Command::Cd(dir) => path_stack.push(dir.as_str()),
            Command::List(size) => {
                dir_size.insert(path_stack.join("|"), *size);
            }
        }
    }

    dir_size
}

#[derive(Debug)]
enum Command {
    CdParent,
    Cd(String),
    List(usize),
}

struct CommandParseErr;

impl std::str::FromStr for Command {
    type Err = CommandParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with(" cd ..") {
            Ok(Command::CdParent)
        } else if s.starts_with(" cd ") {
            let dir = s.trim_start_matches(" cd ").trim();
            Ok(Command::Cd(dir.to_string()))
        } else if s.starts_with(" ls") {
            let lines = s.trim_start_matches(" ls").trim().lines();
            let mut total_size = 0;
            for line in lines {
                if !line.starts_with("dir ") {
                    if let Some((size, _name)) = line.split_once(" ") {
                        total_size += size.parse::<usize>().unwrap();
                    }
                }
            }

            Ok(Command::List(total_size))
        } else {
            Err(CommandParseErr)
        }
    }
}

#[test]
fn test() {
    let input = r#"$ cd /
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
7214296 k"#;

    assert_eq!(95437, part_one(input));
    assert_eq!(24933642, part_two(input));
}
