use std::borrow::Borrow;
use std::cell::RefCell;
use std::fmt::{Display, Formatter, Pointer};
use std::ops::Deref;
use std::rc::Rc;
use crate::NodeType::{Dir, File};

// Advent of code 2022
#[derive(PartialEq, Debug)]
enum NodeType {
    File(u64),
    Dir(u64),
}

type TreeObj = Rc<RefCell<TreeNode>>;

#[derive(PartialEq)]
struct TreeNode {
    name: Option<String>,
    node_type: Option<NodeType>,
    children: Vec<TreeObj>,
    parent: Option<TreeObj>,
}

impl TreeNode {
    fn new() -> Self {
        return TreeNode {
            name: None,
            node_type: None,
            children: vec![],
            parent: None,
        };
    }
}

impl Display for TreeNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "- {} of {} {} \n", self.name.as_ref().unwrap_or(&String::from("unnamed")),
               RefCell::borrow(&self.parent.as_ref()
                   .unwrap_or(&Rc::new(RefCell::new(TreeNode::new()))))
                   .name.as_ref().unwrap_or(&String::from("idk")),
               match &self.node_type.as_ref().unwrap() {
                   File(size) => { size }
                   Dir(size) => { size }
               }
        )?;
        for child in &self.children {
            f.pad("\t")?;
            RefCell::borrow(&child).fmt(f)?
        }
        Ok(())
    }
}


fn build_tree(input: &str) -> TreeObj {
    // let mut current_directory;
    let input: Vec<&str> = input.trim().split('\n').collect();
    let mut dir_tree = TreeNode {
        name: Some(String::from("/")),
        node_type: Some(Dir(0)),
        children: vec![],
        parent: None,
    };
    let mut current_node = Rc::new(RefCell::new(dir_tree));
    let root = Rc::clone(&current_node);
    let mut i = 0;
    while i < input.len() {
        print!("hl\n");
        let line = input[i].trim();
        if !line.starts_with("$ ") {
            panic!("Line should be a command, got {line}");
        }
        println!("Command: {line}");
        // janky, I'm not a fan, but it'll do
        match &line.trim()[0..4] {
            "$ cd" => {
                // cd has no output
                let dest = line.trim().split_once("$ cd ").unwrap().1;
                current_node = match dest {
                    "/" => { Rc::clone(&root) }
                    ".." => {
                        Rc::clone(RefCell::borrow(&current_node).parent.as_ref().unwrap())
                    }
                    _ => {
                        let vodoo_child = Rc::new(RefCell::new(TreeNode {
                            name: Some(String::from(dest)),
                            node_type: Some(Dir(0)),
                            children: vec![],
                            parent: Some(Rc::clone(&current_node)),
                        }));
                        current_node.borrow_mut().children.push(Rc::clone(&vodoo_child));
                        Rc::clone(&vodoo_child)
                    }
                };
                // println!("Current dir = {current_directory}");
            }
            "$ ls" => {
                i += 1;
                while i != input.len() && !input[i].starts_with("$ ") {
                    if !input[i].starts_with("dir ") {
                        let (filesize, filename) = input[i].split_once(' ').unwrap();
                        let filesize = filesize.parse().unwrap();
                        let vodoo_child = Rc::new(RefCell::new(TreeNode {
                            name: Some(String::from(filename)),
                            node_type: Some(File(filesize)),
                            children: vec![],
                            parent: Some(Rc::clone(&current_node)),
                        }));
                        current_node.borrow_mut().children.push(Rc::clone(&vodoo_child));
                    }
                    i += 1;
                }
                i -= 1;
            }
            _ => unreachable!("Why are we here")
        }
        i += 1;
    }
    println!("{}", RefCell::borrow(&root));

    //Why do I have to be so wonky
    root
}

fn calc_dirs(tree: &TreeObj,mut sum :&mut u64) -> u64 {
    let mut sizesum = 0;
    for child in &RefCell::borrow(tree).children {
        sizesum += match &RefCell::borrow(&child).node_type {
            None => { panic!("NO GOD NO") }
            Some(t) => {
                match t {
                    File(size) => { *size }
                    Dir(size) => {
                        if size == &0 {
                            calc_dirs(&child,sum)
                        } else {
                            *size
                        }
                    }
                }
            }
        }
    }
    if sizesum <= 100000{
        println!("{sizesum} {}", RefCell::borrow(&tree));
        *sum += sizesum;
    }
    sizesum
}
fn smallest_deletable(tree: &TreeObj,target: u64, smallest: &mut u64) -> u64 {
    let mut sizesum = 0;
    for child in &RefCell::borrow(tree).children {
        sizesum += match &RefCell::borrow(&child).node_type {
            None => { panic!("NO GOD NO") }
            Some(t) => {
                match t {
                    File(size) => { *size }
                    Dir(size) => {
                        if size == &0 {
                            smallest_deletable(&child,target,smallest)
                        } else {
                            *size
                        }
                    }
                }
            }
        }
    }
    if sizesum >= target && sizesum <= *smallest{
        println!("{sizesum} {}", RefCell::borrow(&tree));
        *smallest = sizesum;
    }
    sizesum
}

fn first_part(input: &str) -> u64 {
    let mut tree = build_tree(input);
    let mut sum =0;
    let totsize = calc_dirs(&tree,&mut sum);
    println!("Sum {sum}");
    // println!("{}", RefCell::borrow(&tree));

    12
}
fn second_part(input:&str) -> u64 {
    let mut tree = build_tree(input);
    let mut sum=0;
    let totsize = calc_dirs(&tree,&mut sum);
    let mut smallest = u64::MAX;
    let current_unused = 70000000 - totsize;
    let target_unused = 30000000;
    let target = target_unused - current_unused;
    smallest_deletable(&tree, target, &mut smallest);
    println!("Smallest size to delete is {smallest}");
    smallest
}

fn main() {
    let test = include_str!("../input/test.txt");
    let input = include_str!("../input/input.txt");
    first_part(test);
    first_part(input);
    second_part(test);
    second_part(input);

    println!("Hello, Rust");
}
