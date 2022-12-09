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

        // This won't work if the file_size is set multiple times
        let mut current_index = dir_index;
        while let Some(next_index) = self.directories[current_index].parent_index {
            self.directories[next_index].files_size += files_size;
            current_index = next_index;
        }
    }
}

fn parse_filesystem(input: &str) -> FileSystem {
    let mut current_dir = 0;
    let mut filesystem = FileSystem::new();

    for command in input.split("$ ").skip(1) {
        let mut lines = command.lines();

        match lines.next().expect("No command.") {
            "ls" => {
                let size = lines
                    .flat_map(|parts| parts.split(" ").next())
                    .flat_map(|first_part| first_part.parse::<u32>().ok())
                    .sum();
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

    let part1 = filesystem
        .directories
        .iter()
        .map(|dir| dir.files_size)
        .filter(|&file_size| file_size <= 100000)
        .sum();

    let total_space = 70000000;
    let target_space = 30000000;
    let used_space = filesystem.directories[0].files_size;
    let need_to_free = used_space - (total_space - target_space);

    let part2 = filesystem.directories.iter()
        .map(|dir| dir.files_size)
        .filter(|&files_size| files_size > need_to_free)
        .min()
        .expect("No answer for part2");

    Ok(Solution::from(part1, part2))
}
