use clap::Parser;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};
use std::str::SplitAsciiWhitespace;

const DISK_SPACE: usize = 70000000;
const REQUIRED_SPACE: usize = 30000000;

fn main() {
    let input = std::fs::read_to_string(Args::parse().filename)
        .expect("Could not read file.")
        .lines()
        .map(TermIo::from_str)
        .collect();

    let fs = Filesystem::new(input);
    println!("Part1: {}", fs.part1());
    println!("Part2: {}", fs.part2());
}

// Run the commands on a fileystem tree structure
struct Filesystem {
    root: Rc<RefCell<Item>>,
    cwd: Rc<RefCell<Item>>,
}

impl Filesystem {
    // New blank filesystem with /
    fn new(input: Vec<TermIo>) -> Filesystem {
        let root = Rc::new(RefCell::new(Item::Dir(Directory::default(String::from(
            "/",
        )))));
        root.borrow_mut().set_parent(Rc::downgrade(&root));

        let mut fs = Filesystem {
            root: Rc::clone(&root),
            cwd: Rc::clone(&root),
        };

        input.iter().for_each(|termio| match termio {
            TermIo::Cmd(command) => fs.exec(command),
            TermIo::Itm(item_builder) => fs.read_item(item_builder),
        });

        fs.root.borrow_mut().calc_size();

        fs
    }

    fn part1(&self) -> usize {
        let mut dirs = vec![];
        self.root.borrow().dir_sizes(&mut dirs);
        dirs.iter().filter(|&&i| i <= 100000).sum()
    }

    fn part2(&self) -> usize {
        let unused_space = DISK_SPACE - self.root.borrow_mut().calc_size();
        let size_goal = REQUIRED_SPACE - unused_space;
        let mut dirs = vec![];
        self.root.borrow().dir_sizes(&mut dirs);
        *dirs.iter().filter(|&&i| i >= size_goal).min().unwrap()
    }

    // Execute a cd command
    fn exec(&mut self, cmd: &Command) {
        match cmd {
            Command::Cd(dir) => {
                self.cwd = match dir.as_str() {
                    "/" => Rc::clone(&self.root),
                    ".." => self.cwd.borrow().get_parent(),
                    name => self
                        .cwd
                        .borrow()
                        .find_subdir(name)
                        .unwrap_or_else(|| self.mkdir(name)),
                }
            }
            Command::Ls => (),
        }
    }

    // Read ls output
    fn read_item(&mut self, item_builder: &ItemBuilder) {
        match item_builder {
            ItemBuilder::Dir(name) => {
                let nodir = self.cwd.borrow().find_subdir(name).is_none();
                nodir.then(|| self.mkdir(name));
            }
            ItemBuilder::Fil(name, size) => {
                let nofile = !self.cwd.borrow().find_subfile(name);
                nofile.then(|| self.touch(name, *size));
            }
        }
    }

    // Create a new dir inside cwd
    fn mkdir(&self, name: &str) -> Rc<RefCell<Item>> {
        let newdir = Rc::new(RefCell::new(Item::Dir(Directory::default(
            name.to_string(),
        ))));
        newdir.borrow_mut().set_parent(Rc::downgrade(&self.cwd));
        self.cwd.borrow_mut().add_child(Rc::clone(&newdir));
        newdir
    }

    // Create a new file inside cwd
    fn touch(&self, name: &str, size: usize) {
        let newfile = Rc::new(RefCell::new(Item::Fil(File::new(name.to_string(), size))));
        self.cwd.borrow_mut().add_child(newfile);
    }
}

// Read ls output
#[derive(Debug)]
enum ItemBuilder {
    Dir(String),
    Fil(String, usize),
}

impl ItemBuilder {
    fn from_split(first: &str, mut split: SplitAsciiWhitespace) -> ItemBuilder {
        let name = split.next().unwrap();
        first.parse().map_or_else(
            |_| ItemBuilder::Dir(name.to_string()),
            |size| ItemBuilder::Fil(name.to_string(), size),
        )
    }
}

// Everything is a file
#[derive(Debug)]
enum Item {
    Dir(Directory),
    Fil(File),
}

impl Item {
    // Set parent of directory
    fn set_parent(&mut self, parent: Weak<RefCell<Item>>) {
        match self {
            Item::Dir(ref mut d) => d.parent = parent,
            Item::Fil(_) => panic!("Can't set parent for a file"),
        }
    }

    // Get parent of directory
    fn get_parent(&self) -> Rc<RefCell<Item>> {
        match self {
            Item::Dir(d) => Rc::clone(&d.parent.upgrade().unwrap()),
            Item::Fil(_) => panic!("Can't get parent for a file"),
        }
    }

    // Find a directory in the children by name
    fn find_subdir(&self, name: &str) -> Option<Rc<RefCell<Item>>> {
        match self {
            Item::Dir(d) => d
                .children
                .iter()
                .find_map(|ptr| match ptr.borrow().deref() {
                    Item::Dir(d) => (d.name == name).then(|| Rc::clone(ptr)),
                    Item::Fil(_) => None,
                }),
            Item::Fil(_) => panic!("Can't find subdirectories of a file"),
        }
    }

    // Check if a file exists in the children
    fn find_subfile(&self, name: &str) -> bool {
        match self {
            Item::Dir(d) => d.children.iter().any(|ptr| match ptr.borrow().deref() {
                Item::Dir(_) => false,
                Item::Fil(f) => f.name == name,
            }),
            Item::Fil(_) => panic!("Can't find subfiles of a file"),
        }
    }

    // Add a child to this directory
    fn add_child(&mut self, child: Rc<RefCell<Item>>) {
        match self {
            Item::Dir(d) => d.children.push(child),
            Item::Fil(_) => panic!("Can't create a directory underneath a file"),
        }
    }

    // Calculate the size of this item
    fn calc_size(&mut self) -> usize {
        match self {
            Item::Dir(d) => d.size.unwrap_or_else(|| {
                let sum: usize = d
                    .children
                    .iter()
                    .map(|ptr| ptr.borrow_mut().calc_size())
                    .sum();

                d.size = Some(sum);
                sum
            }),
            Item::Fil(f) => f.size,
        }
    }

    // get all the directory sizes with a breadth-first search
    fn dir_sizes(&self, dirs: &mut Vec<usize>) {
        match self {
            Item::Dir(d) => {
                dirs.push(d.size.unwrap());
                d.children
                    .iter()
                    .for_each(|ptr| ptr.borrow().dir_sizes(dirs))
            }
            Item::Fil(_) => (),
        }
    }
}

// Terminal i/o is either input command, or output item
#[derive(Debug)]
enum TermIo {
    Cmd(Command),
    Itm(ItemBuilder),
}

impl TermIo {
    fn from_str(line: &str) -> TermIo {
        let mut split = line.split_ascii_whitespace();
        let first = split.next();
        match first {
            Some("$") => TermIo::Cmd(Command::from_split(split)),
            Some(s) => TermIo::Itm(ItemBuilder::from_split(s, split)),
            _ => panic!("Line empty"),
        }
    }
}

// A shell command
#[derive(Debug)]
enum Command {
    Cd(String),
    Ls,
}

impl Command {
    fn from_split(mut split: SplitAsciiWhitespace) -> Command {
        match split.next() {
            Some("cd") => Command::Cd(split.next().unwrap().to_string()),
            Some("ls") => Command::Ls,
            _ => panic!("Command not found"),
        }
    }
}

// A node in the filesystem
#[derive(Debug)]
struct Directory {
    name: String,
    parent: Weak<RefCell<Item>>,
    children: Vec<Rc<RefCell<Item>>>,
    size: Option<usize>,
}

impl Directory {
    fn default(name: String) -> Directory {
        Directory {
            name,
            parent: Weak::new(),
            children: vec![],
            size: None,
        }
    }
}

// A node in the filesystem that contains no info about parent or children
#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}

impl File {
    fn new(name: String, size: usize) -> File {
        File { name, size }
    }
}

#[derive(Parser)]
struct Args {
    filename: String,
}
