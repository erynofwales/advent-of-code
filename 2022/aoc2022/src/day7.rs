use std::collections::HashMap;

const INPUT: &'static str = include_str!("../../Data/day7-input.txt");

#[derive(Debug)]
struct DirectoryStack<'a> {
    stack: Vec<&'a Directory>,
}

impl<'a> DirectoryStack<'a> {
    fn new() -> DirectoryStack<'a> {
        DirectoryStack { stack: Vec::new() }
    }

    fn clear(&mut self) {
        self.stack.clear();
    }

    fn push<'b>(&mut self, dir: &'b Directory)
    where
        'b: 'a,
    {
        self.stack.push(dir);
    }

    fn pop(&mut self) -> Option<&Directory> {
        self.stack.pop()
    }

    fn peek(&mut self) -> Option<&Directory> {
        self.stack.last().cloned()
    }
}

#[derive(Debug, PartialEq)]
struct File {
    name: String,
    size: u32,
}

impl File {
    fn new(name: &str, size: u32) -> File {
        File {
            name: name.to_string(),
            size: size,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Directory {
    name: String,
    directories: HashMap<String, Directory>,
    files: Vec<File>,
}

impl Directory {
    fn new(name: &str) -> Directory {
        Directory {
            name: name.to_string(),
            directories: HashMap::new(),
            files: Vec::new(),
        }
    }

    fn get_subdirectory(&mut self, name: &str) -> Option<&Directory> {
        self.directories.get(name)
    }

    fn add_subdirectory(&mut self, directory_name: &str) -> std::result::Result<(), String> {
        let name = directory_name.to_string();
        if self.directories.contains_key(&name) {
            return Err(format!(
                "'{}' already contains subdirectory '{}'",
                &self.name, directory_name
            ));
        }

        self.directories
            .insert(name, Directory::new(directory_name));

        Ok(())
    }

    fn add_file(&mut self, file: File) {
        self.files.push(file);
    }

    fn size(&self) -> u32 {
        self.files.iter().map(|f| f.size).sum::<u32>()
            + self.directories.values().map(|d| d.size()).sum::<u32>()
    }
}

pub fn main(_filename: &str) -> std::io::Result<()> {
    let line_reader = INPUT.lines();

    let root = Directory::new("/");
    let mut directory_stack = DirectoryStack::new();

    for line in line_reader {
        let (prompt, command) = line.split_at(2);
        if prompt == "$ " {
            let split_command: Vec<&str> = command.split(" ").collect();
            println!("split command: {:?}", split_command);
            match split_command[0] {
                "cd" => {
                    let directory_name = split_command[1];
                    if directory_name == ".." {
                        directory_stack.pop();
                    } else if directory_name == "/" {
                        directory_stack.clear();
                        directory_stack.push(&root);
                    } else {
                        let cwd = directory_stack.peek().expect("No working directory!");
                        let subdirectory = cwd
                            .get_subdirectory(directory_name)
                            .expect(&format!("No subdirectory named '{}'", directory_name));
                        directory_stack.push(&subdirectory);
                    }
                }
                "ls" => {}
                _ => {}
            }
        } else {
            let split_output: Vec<&str> = line.split(" ").collect();
            println!("{:?}", split_output);
            let directory_name = split_output[1];

            if split_output[0] == "dir" {
                dbg!(&directory_stack);
                let cwd = directory_stack.peek().expect("No working directory!");
                println!("Creating subdirectory {:?}", directory_name);
                cwd.add_subdirectory(directory_name);
            } else if let Ok(size) = u32::from_str_radix(split_output[0], 10) {
                let cwd = directory_stack.peek().expect("No working directory!");
                let file = File::new(directory_name, size);
                println!("Appending file {:?} to {:?}", &file, cwd.name);
                cwd.add_file(file);
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::{Directory, DirectoryStack, File};

    #[test]
    fn size_of_empty_directory_is_zero() {
        let root = Directory::new("/");
        assert!(root.size() == 0);
    }

    #[test]
    fn add_subdirectory() {
        let mut root = Directory::new("/");
        assert!(root.add_subdirectory("abc").is_ok());
    }

    #[test]
    fn add_file() {
        let mut root = Directory::new("/");
        assert!(root.size() == 0);

        root.add_file(File::new("abc", 123));
        assert!(root.size() == 123);
    }

    #[test]
    fn init_directory_stack() {
        let _ds = DirectoryStack::new();
    }

    #[test]
    fn add_directory_to_stack() {
        let mut ds = DirectoryStack::new();
        let dir = Directory::new("abc");
        ds.push(&dir);
    }

    #[test]
    fn add_directory_to_stack_and_pop() {
        let mut ds = DirectoryStack::new();
        let dir = Directory::new("abc");
        ds.push(&dir);
        ds.pop();
    }

    #[test]
    fn peek_at_top_of_stack() {
        let mut ds = DirectoryStack::new();
        let dir = Directory::new("abc");
        ds.push(&dir);

        let top = ds.peek();
        assert!(top.is_some());
        assert!(*top.unwrap() == dir);
    }
}
