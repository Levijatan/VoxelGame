use std::{collections::VecDeque, fmt::Debug};

use bevy::core::Byteable;
use bitvec::{array::BitArray, order::Msb0};
use bitvec::field::BitField;

use crate::morton::Morton;

#[derive(Eq, PartialEq)]
enum NodeCommand {
    Filled,
    None,
}
#[derive(Clone, Debug)]
struct Node {
    min: Morton,
    max: Morton,
    leaf_mask: BitArray<Msb0, [u8; 1]>,
    valid_mask: BitArray<Msb0, [u8; 1]>,
    leaves: [Option<Box<Node>>; 8],
}

impl Node {

    pub fn new(min: Morton, max: Morton) -> Self {
        Self {
            min,
            max,
            leaves: [None, None, None, None, None, None, None, None],
            leaf_mask: BitArray::default(),
            valid_mask: BitArray::default(),
        }
    }

    pub fn insert_node(&mut self, val: Morton) {
        self.insert_node_command(val);
    }

    fn insert_node_command(&mut self, val: Morton) -> NodeCommand {
        let total = *self.max - *self.min;
        let tru_pos = *val - *self.min;
        let div = total/8;
        let slot = tru_pos / div;
        if total > 8 {
            if *self.valid_mask.get(slot as usize).unwrap() && !*self.leaf_mask.get(slot as usize).unwrap() {
                if let Some(leaf) = &mut self.leaves[slot as usize] {
                    if leaf.insert_node_command(val) == NodeCommand::Filled {
                        self.leaf_mask.set(slot as usize, true);
                        self.leaves[slot as usize] = None;
                    }
                }
            } else {
                let leaf_min = *self.min + (div * slot);
                let mut leaf = Self::new(Morton(leaf_min), Morton(leaf_min + div));
                leaf.insert_node_command(val);
                self.valid_mask.set(slot as usize, true);
                self.leaves[slot as usize] = Some(Box::new(leaf));
            }
        } else {
            self.leaf_mask.set(slot as usize, true);
            self.valid_mask.set(slot as usize, true);
        }
        if self.leaf_mask.all() {
            NodeCommand::Filled
        } else {
            NodeCommand::None
        }
    }

    fn is_leaf(&self) -> bool {
        self.leaf_mask == self.valid_mask
    }
}

#[derive(Debug)]
pub struct RawSVO {
    tree: Vec<u64>,
    max_depth: u8,
    size: f32,
}

impl RawSVO {
    const fn new(max_depth: u8, size: f32) -> Self {
        Self{
            max_depth,
            size,
            tree: Vec::new(),
        }
    }
}

impl From<SVO> for RawSVO {
    fn from(val: SVO) -> Self {
        let mut out = Self::new(val.max_depth, val.size);
        let mut queue: VecDeque<Node> = VecDeque::new();
        queue.push_back(val.root);
        let mut amount: u32 = 1;
        while let Some(node) = queue.pop_front() {
            let mut raw:BitArray<Msb0, [u64; 1]> = BitArray::default();
            if !node.is_leaf() {
                raw[..15].store(amount);
            }
            raw[24..32].store(node.leaf_mask.as_raw_slice()[0]);
            raw[16..24].store(node.valid_mask.as_raw_slice()[0]);
            out.tree.push(raw.as_raw_slice()[0]);
            for c in &node.leaves {
                if let Some(c_node) = c {
                    queue.push_back(*c_node.clone());
                    amount += 1;
                }
            }
        }
        out
    }
}

unsafe impl Byteable for RawSVO {}

#[derive(Debug)]
pub struct SVO {
    root: Node,
    max_depth: u8,
    size: f32,
}

impl Default for SVO {
    fn default() -> Self {
        Self {
            root: Node::new(Morton(0), Morton(4096)),
            max_depth: 5,
            size: 16.0,
        }
    }
}

impl SVO {
    pub fn insert_node(&mut self, val: Morton) {
        self.root.insert_node(val);
    }

    pub fn build_from_vec(val: Vec<Morton>) -> Self {
        let mut out = Self::default();

        for mort in val {
            out.insert_node(mort);
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use crate::morton;
    use super::*;

    #[test]
    fn tree_build() {
        let mut morton_list = Vec::new();
        for x in 0..1 {
            for y in 0..1 {
                for z in 0..1 {
                    morton_list.push(morton::encode(x, y, z));
                }
            }
        }

        let tree = SVO::build_from_vec(morton_list);
        println!("{:?}", tree);
    }

    #[test]
    fn tree_build_raw() {
        let mut morton_list = Vec::new();
        for x in 0..8 {
            for y in 0..16 {
                for z in 0..10 {
                    morton_list.push(morton::encode(x, y, z));
                }
            }
        }

        let tree: RawSVO = SVO::build_from_vec(morton_list).into();
        println!("{:?}", tree);
    }
}