use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alpha1, digit1},
    multi::many1,
    IResult,
};

#[derive(Debug)]
enum Line {
    Cd(Cd),
    Ls(Ls),
    DirectoryOutput(DirectoryOutput),
    FileOutput(FileOutput),
}

#[derive(Debug)]
enum Cd {
    AbsolutePath(String),
    RelativePath(String),
    PrevDirectory,
    RootDirectory,
}

impl Cd {
    fn from_path(path: &str) -> Self {
        if path == "/" {
            Self::RootDirectory
        } else if path == ".." {
            Self::PrevDirectory
        } else if path.starts_with("/") {
            Self::AbsolutePath(path.to_string())
        } else {
            Self::RelativePath(path.to_string())
        }
    }
}

#[derive(Debug)]
struct Ls {}

#[derive(Debug)]
struct DirectoryOutput(String);

impl DirectoryOutput {
    fn from_path(path: &str) -> Self {
        Self(path.to_string())
    }
}

#[derive(Debug)]
struct FileOutput {
    name: String,
    size: usize,
}

fn parse_ls(input: &str) -> IResult<&str, Line> {
    let (input, _line) = tag("ls")(input)?;
    Ok((input, Line::Ls(Ls {})))
}

fn parse_cd(input: &str) -> IResult<&str, Line> {
    let (input, _) = tag("cd ")(input)?;
    let (input, path) = alt((tag("/"), tag(".."), alpha1))(input)?;
    Ok((input, Line::Cd(Cd::from_path(path))))
}

fn parse_directory_output(input: &str) -> IResult<&str, Line> {
    let (input, _) = tag("dir ")(input)?;
    let (input, path) = alpha1(input)?;
    Ok((
        input,
        Line::DirectoryOutput(DirectoryOutput::from_path(path)),
    ))
}

fn parse_file_output(input: &str) -> IResult<&str, Line> {
    let (input, size) = digit1(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, name) = take_until("\n")(input)?;
    Ok((
        input,
        Line::FileOutput(FileOutput {
            name: name.to_string(),
            size: size.parse().unwrap(),
        }),
    ))
}

fn parse_command(input: &str) -> IResult<&str, Line> {
    let (input, _) = tag("$ ")(input)?;
    let (input, command) = alt((parse_cd, parse_ls))(input)?;
    Ok((input, command))
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, line) = alt((parse_command, parse_directory_output, parse_file_output))(input)?;
    let (input, _) = tag("\n")(input)?;
    Ok((input, line))
}

fn parse_lines(input: &str) -> IResult<&str, Vec<Line>> {
    let (input, lines) = many1(parse_line)(input)?;
    Ok((input, lines))
}

#[derive(Debug)]
struct Dir {
    name: String,
    files: Vec<File>,
    dirs: Vec<Dir>,
    size: usize,
}

impl Dir {
    // Insert into the directory tree
    // Updating the size of the directory and all parent directories
    fn insert_dir(self: &mut Self, path: &str) {
        if self.dirs.iter().any(|d| d.name == path) {
            return;
        }
        self.dirs.push(Dir {
            name: path.to_string(),
            files: vec![],
            dirs: vec![],
            size: 0,
        });
    }

    fn find_dir(self: &mut Self, dir: &str) -> Option<&mut Dir> {
        for d in self.dirs.iter_mut() {
            if d.name == dir {
                return Some(d);
            }
        }
        None
    }

    fn insert_file(self: &mut Self, name: &str, size: usize) {
        self.files.push(File {
            name: name.to_string(),
            size,
        });
    }

    fn update_sizes(self: &mut Self) -> usize {
        self.size = self.files.iter().map(|f| f.size).sum();
        self.dirs
            .iter_mut()
            .for_each(|d| self.size += d.update_sizes());
        self.size
    }

    #[allow(dead_code)]
    fn ls(&self, indent: i32) {
        println!("{}-{}: {}", " ".repeat(indent as usize), self.name, self.size);
        for d in self.dirs.iter() {
            d.ls(indent + 1);
        }
        for f in self.files.iter() {
            println!(
                "{}*{}: {}",
                " ".repeat((indent + 1) as usize),
                f.name,
                f.size
            );
        }
    }

    fn flatten_dirs(&self, path: &str) -> Vec<(String, usize)> {
        let new_path = if path == "" {
            self.name.clone()
        } else {
            format!("{}/{}",path.to_owned(), self.name)
        };
        let mut dirs = vec![(new_path.clone(), self.size)];
        for d in self.dirs.iter() {
            dirs.extend(d.flatten_dirs(&new_path.clone()));
        }
        dirs
    }
}

#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}

#[derive(Debug)]
struct State {
    current_dir: Vec<String>,
    tree: Dir,
}

impl State {
    fn new() -> State {
        State {
            tree: Dir {
                name: "C:".to_string(),
                files: vec![],
                dirs: vec![],
                size: 0,
            },
            current_dir: vec![],
        }
    }

    fn pwd_obj(self: &mut Self) -> &mut Dir {
        let mut current_dir = &mut self.tree;
        for dir in self.current_dir.iter() {
            current_dir = current_dir.find_dir(dir).unwrap();
        }
        current_dir
    }

    fn move_or_create_dir(self: &mut Self, cd: &Cd) {
        match cd {
            Cd::AbsolutePath(path) => {
                todo!("{:?}", path)
            }
            Cd::RelativePath(path) => {
                path.split("/").for_each(|p| {
                    if p == ".." {
                        self.current_dir.pop();
                    } else {
                        self.pwd_obj().insert_dir(p);
                        self.current_dir.push(p.to_string());
                    }
                });
            }
            Cd::PrevDirectory => {
                self.current_dir.pop();
            }
            Cd::RootDirectory => {
                self.current_dir = vec![];
            }
        }
    }

    fn create_dir(self: &mut Self, d: &DirectoryOutput) {
        self.pwd_obj().insert_dir(&d.0);
    }

    fn create_file(self: &mut Self, f: &FileOutput) {
        self.pwd_obj().insert_file(&f.name, f.size);
    }

    fn process_command(self: &mut Self, line: &Line) {
        match line {
            Line::Cd(cd) => self.move_or_create_dir(cd),
            Line::Ls(_) => (),
            Line::DirectoryOutput(d) => self.create_dir(d),
            Line::FileOutput(f) => self.create_file(f),
        }
    }
}

pub fn process_part_1(input: &str) -> usize {
    let (_input, lines) = parse_lines(input).unwrap();
    let mut state = State::new();
    for line in lines.iter() {
        state.process_command(line);
    }
    state.tree.update_sizes();
    let total = state.tree.flatten_dirs("").into_iter().filter(|d| d.1 <= 100000).map(|d| d.1).sum::<usize>();
    println!("Total {}", total);
    total
}

pub fn process_part_2(input: &str) -> usize {
    let (_input, lines) = parse_lines(input).unwrap();
    let mut state = State::new();
    for line in lines.iter() {
        state.process_command(line);
    }
    state.tree.update_sizes();
    let space_available = 70000000 - state.tree.size;
    let space_needed = 30000000 - space_available;
    let smallest = state.tree.flatten_dirs("")
        .into_iter()
        .filter(|d| d.1 >= space_needed)
        .min_by_key(|d| d.1);
    println!("Smallest deletable {:?}", smallest);
    smallest.unwrap().1
}

#[cfg(test)]
mod test {
    #[test]
    fn test_1() {
        let input = "$ cd /
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
7214296 k
";
        assert_eq!(crate::process_part_1(input), 95437);
    }
    #[test]
    fn test_2() {
        let input = "$ cd /
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
7214296 k
";
        assert_eq!(crate::process_part_2(input), 24933642);
    }
}
