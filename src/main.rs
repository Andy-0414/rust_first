use std::{io, result};

struct BSTNode {
    value: i32,
    left: Option<Box<BSTNode>>,
    right: Option<Box<BSTNode>>,
}

impl BSTNode {
    pub fn new(value: i32) -> Self {
        return BSTNode {
            value,
            left: None,
            right: None,
        };
    }

    pub fn insert(&mut self, value: i32) {
        let mut currentTree = self;
        let mut nextTree;

        loop {
            if (currentTree.value == value) {
                break;
            }

            nextTree = if (currentTree.value > value) {
                &mut currentTree.left
            } else {
                &mut currentTree.right
            };

            match *nextTree {
                None => {
                    *nextTree = Some(Box::new(BSTNode::new(value)));
                    break;
                }
                Some(ref mut subTree) => currentTree = subTree,
            }
        }
        // unsafe {
        //     let mut parentTree = currentTree.parent;
        //     loop {
        //         match parentTree {
        //             None => {
        //                 return;
        //             }
        //             Some(subTree) => {
        //                 parentTree = (*subTree).parent;
        //                 if ((*subTree).depth <= depth) {
        //                     (*subTree).depth = depth;
        //                     println!("{}, {}", (*subTree).value, (*subTree).depth)
        //                 }
        //             }
        //         }
        //     }
        // }
    }

    pub fn search(&mut self, value: i32) -> bool {
        let mut currentTree = self;
        let mut nextTree;

        loop {
            if (currentTree.value == value) {
                return true;
            }

            nextTree = if (currentTree.value > value) {
                &mut currentTree.left
            } else {
                &mut currentTree.right
            };

            match *nextTree {
                None => {
                    return false;
                }
                Some(ref mut subTree) => currentTree = subTree,
            }
        }
    }

    pub fn print(self) {}
}

fn main() {
    let mut bstTree = BSTNode::new(10);

    bstTree.insert(1);
    bstTree.insert(3);
    bstTree.insert(5);
    bstTree.insert(7);
    bstTree.insert(9);

    bstTree.print()

    // println!("Hello, world!");
    // let mut input = String::new();

    // io::stdin()
    //     .read_line(&mut input)
    //     .expect("Failed to read line");

    // println!("RESULT: {}", input);
}
