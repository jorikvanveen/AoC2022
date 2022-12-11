use crate::terminal::*;

pub type FsIndex = usize;

#[derive(Debug)]
pub struct Directory {
    name: String,
    children: Vec<FsIndex>,
    parent: FsIndex
}

#[derive(Debug)]
pub struct File {
    name: String,
    size: usize,
    parent: FsIndex
}

#[derive(Debug)]
pub enum FsNode {
    Directory (Directory) ,
    File (File)
}

impl FsNode {
    pub fn get_parent(&self) -> FsIndex {
        match self {
            FsNode::File(file) => file.parent,
            FsNode::Directory(d) => d.parent
        }
    }

    pub fn get_child_by_name(&self, fs: &Filesystem, name: &str) -> Option<FsIndex> {
        match self {
            FsNode::File(_) => None,
            FsNode::Directory(d) => {
                for child_idx in d.children.iter() {
                    let node = fs.get_node(*child_idx);
                    if node.get_name() == name {
                        return Some(*child_idx)    
                    }
                }

                return None
            }
        }
    }

    pub fn get_name(&self) -> &str {
        match self {
            FsNode::Directory(d) => &d.name,
            FsNode::File(f) => &f.name
        }
    }

    pub fn get_size(&self, fs: &Filesystem) -> usize {
        match self {
            FsNode::File(f) => f.size,
            FsNode::Directory(d) => {
                let mut total = 0;

                for child_idx in d.children.iter() {
                    let child = fs.get_node(*child_idx);
                    total += child.get_size(fs);
                };

                total
            }
        }
    }

    pub fn is_directory(&self) -> bool {
        match self {
            FsNode::Directory(_) => true,
            FsNode::File(_) => false
        }
    }
}

#[derive(Debug)]
pub struct Filesystem {
    nodes: Vec<FsNode>,
    root: FsIndex
}

impl Filesystem {
    pub fn from_commands(cmds: Vec<Command>) -> Filesystem {
        let mut nodes: Vec<FsNode> = vec![];    
        let root = FsNode::Directory(Directory {
            name: "/".into(),
            children: vec![],
            parent: 0 // cd .. in / goes back to /
        });
        nodes.push(root);

        let mut fs = Filesystem { nodes, root: 0 };

        // Working directory
        let mut wd: FsIndex = 0;

        for cmd in cmds {
            match cmd {
                Command::Cd { argument } => {
                    match argument.as_str() {
                        ".." => { wd = fs.get_node(wd).get_parent() },
                        "/" => {},
                        _ => {
                            // Get index from children of working directory
                            wd = fs.get_node(wd).get_child_by_name(&fs, &argument).unwrap();
                        }
                    }
                },
                Command::Ls { output } => {
                    for output_line in output {
                        let new_idx = fs.new_index();

                        // Check if already exists
                        if fs.get_node(wd).get_child_by_name(&fs, output_line.get_name()).is_some() {
                            println!("Directory already exists");
                            continue;
                        }

                        {
                            let parent_node = fs.get_node_mut(wd);
                            match parent_node {
                                FsNode::Directory(d) => d.children.push(new_idx),
                                _ => panic!("Not a directory")
                            }
                        }

                        match output_line {
                            OutputLine::Directory { name } => {
                                let new_node = FsNode::Directory(Directory {
                                    name,
                                    children: vec![],
                                    parent: wd
                                });

                                fs.add_node(new_node);
                            },
                            OutputLine::File { name, size } => {
                                let new_node = FsNode::File(File {
                                    name,
                                    size,
                                    parent: wd
                                });

                                fs.add_node(new_node);
                            }
                        }
                    }
                }
            }
        }

        fs
    }

    pub fn get_node(&self, idx: FsIndex) -> &FsNode {
        &self.nodes[idx]
    }

    pub fn get_node_mut(&mut self, idx: FsIndex) -> &mut FsNode {
        &mut self.nodes[idx]
    }

    pub fn add_node(&mut self, node: FsNode) {
        self.nodes.push(node);
    }

    pub fn new_index(&self) -> FsIndex {
        self.nodes.len()
    }

    pub fn get_nodes(&self) -> &Vec<FsNode> {
        &self.nodes
    }
    
    pub fn get_root(&self) -> FsIndex {
        self.root
    }
}
