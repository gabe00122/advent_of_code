use crate::challenge_result::{ChallengeResult, Solution};

struct Directory<'a> {
    name: &'a str,
    parent_index: Option<usize>,
    files_size: u32,
    children_indices: Vec<usize>,
}

impl<'a> Directory<'a> {
    fn new(name: &'a str, parent_index: Option<usize>) -> Directory {
        Directory {
            name,
            parent_index,
            files_size: 0,
            children_indices: Vec::new(),
        }
    }
}

struct FileSystem<'a> {
    directories: Vec<Directory<'a>>,
}

impl<'a> FileSystem<'a> {
    fn new() -> FileSystem<'a> {
        FileSystem {
            directories: vec![Directory::new("/", None)],
        }
    }

    fn get_child_dir(&self, parent_index: usize, name: &str) -> Option<usize> {
        self.directories[parent_index]
            .children_indices
            .iter()
            .cloned()
            .find(|&index| self.directories[index].name == name)
    }

    fn make_dir(&mut self, parent_index: usize, name: &'a str) -> usize {
        let next_directory_index = self.directories.len();

        self.directories[parent_index]
            .children_indices
            .push(next_directory_index);

        self.directories
            .push(Directory::new(name, Some(parent_index)));

        next_directory_index
    }

    fn get_or_make_dir(&mut self, parent_index: usize, name: &'a str) -> usize {
        if let Some(child_index) = self.get_child_dir(parent_index, name) {
            child_index
        } else {
            self.make_dir(parent_index, name)
        }
    }

    fn get_parent(&self, dir_index: usize) -> Option<usize> {
        self.directories[dir_index].parent_index
    }

    fn set_dir_size(&mut self, dir_index: usize, files_size: u32) {
        self.directories[dir_index].files_size = files_size;
    }
}

fn parse_filesystem(input: &str) -> FileSystem {
    let mut current_dir = 0;
    let mut filesystem = FileSystem::new();

    for command in input.split("$ ").skip(1) {
        let mut lines = command.lines();

        match lines.next().expect("No command.") {
            "ls" => {
                let size = lines.flat_map(| part | part.parse::<u32>().ok()).sum();
                filesystem.set_dir_size(current_dir, size)
            }
            "cd /" => {
                current_dir = 0;
            }
            "cd .." => {
                if let Some(next_dir) = filesystem.get_parent(current_dir) {
                    current_dir = next_dir
                }
            }
            cd => {
                current_dir = filesystem.get_or_make_dir(current_dir, &cd[3..]);
            }
        }
    }

    filesystem
}

pub fn run(input: &str) -> ChallengeResult {
    let filesystem = parse_filesystem(input);

    let (size, count) = count_large_directories(&filesystem, 0);

    println!("size: {}", size);
    println!("count: {}", count);

    Ok(Solution::from(filesystem.directories.len(), 0))
}

fn count_large_directories(filesystem: &FileSystem, directory_index: usize) -> (u32, u32) {
    let (children_size, children_count) = filesystem.directories[directory_index]
        .children_indices
        .iter()
        .fold((0, 0), |(accum_size, accum_count), &child_dir_index| {
            let (child_size, child_count) = count_large_directories(filesystem, child_dir_index);

            (accum_size + child_size, accum_count + child_count)
        });

    let total_size = filesystem.directories[directory_index].files_size + children_size;
    let total_count = if total_size <= 100000 { total_size } else { 0 } + children_count;

    (total_size, total_count)
}

fn count_large_directories2(filesystem: &FileSystem, directory_index: usize) -> (u32, u32) {
    let (children_size, children_count) = filesystem.directories[directory_index]
        .children_indices
        .iter()
        .fold((0, 0), |(accum_size, accum_count), &child_dir_index| {
            let (child_size, child_count) = count_large_directories2(filesystem, child_dir_index);

            (accum_size + child_size, accum_count + child_count)
        });

    let total_size = filesystem.directories[directory_index].files_size + children_size;
    let total_count = if total_size <= 100000 { total_size } else { 0 } + children_count;

    (total_size, total_count)
}
