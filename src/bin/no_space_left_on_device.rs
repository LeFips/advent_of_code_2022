use std::io::{BufRead, BufReader, Read, Seek};

fn main() {
    let mut input_reader: BufReader<std::fs::File> = advent_of_code_2022::get_reader_from_arg();
    let mut fs = build_fs(&mut input_reader);
    fs.calc_ele_size(0);

    let part1: usize = fs
        .elements
        .iter()
        .filter_map(|ele| {
            if ele.ele_type == FsEleType::Directory {
                if ele.size <= 100000 {
                    return Some(ele.size);
                }
            }
            None
        })
        .sum();
        
    let total_space: usize = 70000000;
    let needed_space: usize = 30000000;
    let used_space: usize = fs.elements[0].size;
    
    let part2: usize = fs
        .elements
        .iter()
        .filter_map(|ele| if ele.ele_type == FsEleType::Directory { Some(ele.size) } else { None })
        .filter(|size| total_space - used_space + size > needed_space)
        .min()
        .unwrap_or(0);

    println!("Part1: {part1}, Part2: {part2}");
}

fn build_fs(input_reader: &mut BufReader<std::fs::File>) -> FileSystem {
    let max_possible_size: usize = input_reader
        .by_ref()
        .lines()
        .filter(|line| !line.as_ref().unwrap().starts_with("$"))
        .count();

    input_reader.rewind().unwrap();
    let mut file_system: FileSystem = FileSystem::new(String::from("/"), max_possible_size);
    for line in input_reader.lines() {
        let line: String = line.unwrap();
        let segments: Vec<&str> = line.split(" ").collect();

        if segments[0] == "$" {
            if segments[1] == "cd" {
                match segments[2] {
                    "/" => file_system.goto_root(),
                    ".." => file_system.goto_parent(),
                    dir_name => {
                        if let Some(ele_id) = file_system.get_child(dir_name, FsEleType::Directory)
                        {
                            file_system.goto(ele_id);
                        } else {
                            let new_id: usize = file_system.add_directory(dir_name);
                            file_system.goto(new_id);
                        }
                    }
                }
            }
        } else if segments[0] == "dir" {
            file_system.add_directory(segments[1]);
        } else {
            let size: usize = segments[0].parse().unwrap();
            file_system.add_file(segments[1], size);
        }
    }
    calc_dir_sizes(&mut file_system);
    file_system
}

fn calc_dir_sizes(file_system: &mut FileSystem) {
    file_system.goto_root();
}

struct FileSystem {
    curr_dir: usize,
    elements: Vec<FsElement>,
}

impl FileSystem {
    fn new(root: String, capacity: usize) -> Self {
        let mut fs: FileSystem = FileSystem {
            curr_dir: 0,
            elements: Vec::with_capacity(capacity),
        };
        fs.elements.push(FsElement {
            name: root,
            ele_type: FsEleType::Directory,
            size: 0,
            parent: None,
            next_sibling: None,
            first_child: None,
            last_child: None,
        });
        fs
    }

    fn goto_root(&mut self) {
        self.curr_dir = 0;
    }

    fn goto_parent(&mut self) {
        self.curr_dir = self.elements[self.curr_dir].parent.unwrap_or(0);
    }

    fn goto(&mut self, ele_id: usize) {
        if let FsEleType::Directory = self.elements[ele_id].ele_type {
            self.curr_dir = ele_id;
        } else {
            panic!("Can't goto a file");
        }
    }

    fn get_child(&self, name: &str, ele_type: FsEleType) -> Option<usize> {
        if let Some(mut child_id) = self.elements[self.curr_dir].first_child {
            loop {
                let ele: &FsElement = &self.elements[child_id];

                if ele_type == ele.ele_type {
                    if ele.name == name {
                        return Some(child_id);
                    }
                }
                if let Some(sibling) = self.elements[child_id].next_sibling {
                    child_id = sibling;
                } else {
                    break;
                }
            }
        }
        None
    }

    fn add_directory(&mut self, new_dir_name: &str) -> usize {
        let new_id: usize = self.elements.len();

        if let Some(existing_id) = self.get_child(new_dir_name, FsEleType::Directory) {
            return existing_id;
        }

        if let Some(last_child) = self.elements[self.curr_dir].last_child {
            self.elements[last_child].next_sibling = Some(new_id);
        } else {
            self.elements[self.curr_dir].first_child = Some(new_id);
        }
        self.elements[self.curr_dir].last_child = Some(new_id);

        self.elements.push(FsElement {
            name: String::from(new_dir_name),
            parent: Some(self.curr_dir),
            ele_type: FsEleType::Directory,
            size: 0,
            next_sibling: None,
            first_child: None,
            last_child: None,
        });
        new_id
    }

    fn add_file(&mut self, name: &str, size: usize) {
        let new_id: usize = self.elements.len();

        if let Some(_) = self.get_child(name, FsEleType::File) {
            return ();
        }

        if let Some(last_child) = self.elements[self.curr_dir].last_child {
            self.elements[last_child].next_sibling = Some(new_id);
        } else {
            self.elements[self.curr_dir].first_child = Some(new_id);
        }
        self.elements[self.curr_dir].last_child = Some(new_id);

        self.elements.push(FsElement {
            name: String::from(name),
            parent: Some(self.curr_dir),
            size: size,
            ele_type: FsEleType::File,
            next_sibling: None,
            first_child: None,
            last_child: None,
        });
    }

    fn calc_ele_size(&mut self, id: usize) -> usize {
        if self.elements[id].size != 0 {
            return self.elements[id].size;
        } else {
            let mut size: usize = 0;
            if let Some(mut child_id) = self.elements[id].first_child {
                loop {
                    size += self.calc_ele_size(child_id);
                    
                    if let Some(sibling) = self.elements[child_id].next_sibling {
                        child_id = sibling;
                    } else {
                        break;
                    }
                }
            }
            self.elements[id].size = size;
            size
        }
    }
}

#[derive(PartialEq)]
enum FsEleType {
    File,
    Directory,
}

struct FsElement {
    name: String,
    ele_type: FsEleType,
    size: usize,
    parent: Option<usize>,
    next_sibling: Option<usize>,
    first_child: Option<usize>,
    last_child: Option<usize>,
}
