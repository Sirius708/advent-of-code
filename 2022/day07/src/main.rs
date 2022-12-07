use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Directory {
    name: String,
    entries: Vec<FileOrDirectory>,
}

impl Directory {
    fn find_child_dir_mut(&mut self, name: &str) -> Option<&mut Directory> {
        self.entries
            .iter_mut()
            .filter_map(|e| {
                if let FileOrDirectory::Directory(dir) = e {
                    Some(dir)
                } else {
                    None
                }
            })
            .find(|dir| dir.name == name)
    }

    fn get_size(&self) -> u32 {
        let mut size = 0;
        for entry in &self.entries {
            match entry {
                FileOrDirectory::Directory(dir) => size += dir.get_size(),
                FileOrDirectory::File(file) => size += file.size,
            }
        }
        size
    }

    fn walk_file_tree<W>(&self, walker: &W)
    where
        W: Fn(&FileOrDirectory),
    {
        for entry in &self.entries {
            walker(entry);
            if let FileOrDirectory::Directory(dir) = entry {
                dir.walk_file_tree(walker);
            }
        }
    }
}

#[derive(Debug)]
struct File {
    name: String,
    size: u32,
}

#[derive(Debug)]
enum FileOrDirectory {
    Directory(Directory),
    File(File),
}

#[derive(Debug)]
enum Command {
    ChangeDirectory { target: ChangeDirectoryTarget },
    ListFiles { output: Vec<ListOutput> },
}

#[derive(Debug)]
enum ChangeDirectoryTarget {
    Root,
    Parent,
    Directory(String),
}

#[derive(Debug)]
enum ListOutput {
    Directory(String),
    File(String, u32),
}

fn parse_history(mut lines: &[String]) -> Vec<Command> {
    let mut commands = vec![];
    while !lines.is_empty() {
        let (remaining_lines, command) = parse_command(lines);
        lines = remaining_lines;
        commands.push(command);
    }
    commands
}

fn parse_command(lines: &[String]) -> (&[String], Command) {
    let cmd_args = lines[0][2..].split_ascii_whitespace().collect::<Vec<_>>();
    match cmd_args[0] {
        "cd" => (
            &lines[1..],
            Command::ChangeDirectory {
                target: match cmd_args[1] {
                    "/" => ChangeDirectoryTarget::Root,
                    ".." => ChangeDirectoryTarget::Parent,
                    target => ChangeDirectoryTarget::Directory(target.to_owned()),
                },
            },
        ),
        "ls" => {
            let mut output = vec![];
            let mut index = 1;
            while index < lines.len() && !lines[index].starts_with('$') {
                output.push(parse_list_output(&lines[index]));
                index += 1;
            }
            (&lines[index..], Command::ListFiles { output })
        }
        _ => unreachable!(),
    }
}

fn parse_list_output(line: &str) -> ListOutput {
    let (left, right) = line.split_once(' ').unwrap();
    match left {
        "dir" => ListOutput::Directory(right.to_owned()),
        size => ListOutput::File(right.to_owned(), size.parse().unwrap()),
    }
}

fn convert_history_to_file_tree(history: &[Command]) -> Directory {
    let mut current_path: Vec<String> = vec![];
    let mut file_tree = Directory {
        name: "/".to_owned(),
        entries: vec![],
    };
    let mut current_file = &mut file_tree;

    for command in history {
        match command {
            Command::ChangeDirectory { target } => match target {
                ChangeDirectoryTarget::Root => {
                    current_path.clear();
                    current_file = &mut file_tree;
                }
                ChangeDirectoryTarget::Parent => {
                    current_path.pop();
                    current_file = &mut file_tree;
                    for dir_name in &current_path {
                        current_file = current_file.find_child_dir_mut(dir_name).unwrap();
                    }
                }
                ChangeDirectoryTarget::Directory(name) => {
                    current_path.push(name.to_owned());
                    current_file = current_file.find_child_dir_mut(name).unwrap();
                }
            },
            Command::ListFiles { output } => {
                for line in output {
                    match line {
                        ListOutput::Directory(name) => {
                            current_file
                                .entries
                                .push(FileOrDirectory::Directory(Directory {
                                    name: name.to_owned(),
                                    entries: vec![],
                                }));
                        }
                        ListOutput::File(name, size) => {
                            current_file.entries.push(FileOrDirectory::File(File {
                                name: name.to_owned(),
                                size: *size,
                            }));
                        }
                    }
                }
            }
        }
    }
    file_tree
}

fn main() {
    let input_lines = util::get_input_lines();
    let commands = parse_history(&input_lines);
    let file_tree = convert_history_to_file_tree(&commands);

    let dir_sizes = Rc::new(RefCell::new(vec![]));
    file_tree.walk_file_tree(&|e| {
        if let FileOrDirectory::Directory(dir) = e {
            let size = dir.get_size();
            dir_sizes.borrow_mut().push(size);
        }
    });
    dir_sizes.borrow_mut().sort_unstable();
    println!(
        "Sum of directory sizes: {}",
        dir_sizes
            .borrow()
            .iter()
            .take_while(|s| **s <= 100_000)
            .sum::<u32>()
    );

    const TOTAL_DISK_SPACE: u32 = 70000000;
    const NEEDED_SPACE: u32 = 30000000;
    let total_used_space = file_tree.get_size();
    let remaining_space = TOTAL_DISK_SPACE - total_used_space;
    let space_to_free = NEEDED_SPACE - remaining_space;
    println!(
        "Minimum space to free: {}",
        dir_sizes
            .borrow()
            .iter()
            .find(|s| **s >= space_to_free)
            .unwrap()
    );
}
