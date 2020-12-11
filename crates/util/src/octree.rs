use super::morton::Morton;
use std::convert::TryInto;
use std::cmp::Ordering;

#[derive(Copy, Clone, Debug)]
pub struct LeafMeta<M> {
    coord: Morton,
    meta: M,
}

impl<M> LeafMeta<M> {
    pub const fn new(meta: M, coord: Morton) -> Self {
        Self {
            coord,
            meta
        }
    }
}

impl<M: Eq> PartialOrd for LeafMeta<M> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.coord.partial_cmp(&other.coord)
    }
}

impl<M: Eq> PartialEq for LeafMeta<M> {
    fn eq(&self, other: &Self) -> bool {
        self.coord == other.coord || self.meta == other.meta
    }
}

impl<M: Eq> Eq for LeafMeta<M> {}
impl<M: Eq> Ord for LeafMeta<M> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.coord.cmp(&other.coord)
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum NodeType<M: Eq> {
    Parent([usize; 8]),
    Leaf(LeafMeta<M>),
    Empty,
}

impl<M: Eq> Default for NodeType<M> {
    fn default() -> Self {
        Self::Empty
    }
}

impl<M: Eq> NodeType<M> {
    fn is_empty(&self) -> bool{
        matches!(*self, Self::Empty)
    }
}

#[derive(Debug)]
pub struct SparseVoxelOctree<M: Eq>{
    tree: Vec<NodeType<M>>,
    max_depth: u8,
}

impl<M: Eq> Default for SparseVoxelOctree<M> {
    fn default() -> Self {
        Self{
            max_depth: 5,
            tree: Vec::new(),
        }
    }
}

impl<M: Eq + Default + Copy + std::fmt::Debug> SparseVoxelOctree<M> {
    pub fn build_from(mut val: Vec<LeafMeta<M>>) -> Self {
        let mut out = Self::default();
        let max_leaves = out.max_leaves();
        assert!(val.len() <= max_leaves.try_into().unwrap());
        val.sort();

        let mut buf_builder = BuildBuffer::new(out.max_depth().into());
        let mut buf = NodeBuf::default();
        let mut at = 0;
        for meta in val {
            if at != *meta.coord {
                for _ in at..*meta.coord {
                    if buf.add(NodeType::Empty) {
                        buf_builder.read_in_voxels(&buf);
                        buf.drain();
                    }
                }
                at = *meta.coord;
                if buf.add(NodeType::Leaf(meta)) {
                    buf_builder.read_in_voxels(&buf);
                    buf.drain();
                }
            } else if buf.add(NodeType::Leaf(meta)) {
                buf_builder.read_in_voxels(&buf);
                buf.drain();
            }
            at += 1;
            assert!(at <= out.max_leaves().into());
        }
        if at < max_leaves.into() {
            for _ in at..max_leaves.into() {
                if buf.add(NodeType::Empty) {
                    buf_builder.read_in_voxels(&buf);
                    buf.drain();
                }
            }
        }

        out.tree = buf_builder.finish();
        out
    }

    fn max_leaves(&self) -> u32 {
        let out: u32 = 8;
        out.pow((self.max_depth() - 1).into())
        
    }

    fn max_depth(&self) -> u8 {
        self.max_depth
    }
}

#[derive(Default)]
struct NodeBuf<M: Eq>([NodeType<M>; 8], usize);

impl<M: Eq + Copy> NodeBuf<M> {
    fn is_empty(&self) -> bool {
        self.1 == 0
    }

    fn add(&mut self, val: NodeType<M>) -> bool {
        let at = self.1;
        self.0[at] = val;
        self.1 += 1;
        self.1 == 8
    }

    fn drain(&mut self) -> [NodeType<M>; 8] {
        self.1 = 0;
        let out = self.0;
        self.0 = [NodeType::Empty; 8];
        out
    }

    fn filled(&self) -> bool {
        if let NodeType::Leaf(first) = self.0[0] {
            for n in &self.0 {
                if let NodeType::Leaf(m) = n {
                    if *m != first {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }
}

struct BuildBuffer<M: Eq> {
    buffers: Vec<NodeBuf<M>>,
    reverse_tree: Vec<NodeType<M>>,
}

impl<M: Eq + Default + Copy> BuildBuffer<M> {

    fn new(max_depth: usize) -> Self {
        let mut buffers = Vec::new();
        for _i in 1..max_depth {
            buffers.push(NodeBuf::default());
        }
        Self{
            buffers,
            reverse_tree: Vec::new(),
        }
    }
    
    fn read_in_voxels(&mut self, val: &NodeBuf<M>) {
        if val.is_empty() {
            self.add_parent(NodeType::Empty, 0);
        } else if val.filled() {
            self.add_parent(val.0[0], 0);
        }  else {
            let mut parent_arr = [0; 8];
            let mut empty = true;
            for i in (0..8).rev() {
                let n = val.0[i];
                if !n.is_empty() {
                    self.reverse_tree.push(n);
                    parent_arr[i] = self.reverse_tree.len();
                    empty = false;
                }
            }
            if empty {
                self.add_parent(NodeType::Empty, 0);
            } else {
                self.add_parent(NodeType::Parent(parent_arr), 0);
            }
        }
    }

    fn add_parent(&mut self, val: NodeType<M>, depth: usize) {
        let buf = &mut self.buffers[depth];
        if buf.add(val) {
            if buf.filled() {
                let l = buf.drain();
                self.add_parent(l[0], depth + 1);
            } else {
                let mut parent_arr = [0; 8];
                let b = buf.drain();
                let mut empty = true;
                for i in (0..8).rev() {
                    let n = b[i];
                    if !n.is_empty() {
                        self.reverse_tree.push(n);
                        parent_arr[i] = self.reverse_tree.len();
                        empty = false;
                    }
                }
                if empty {
                    self.add_parent(NodeType::Empty, depth + 1);
                } else {
                    self.add_parent(NodeType::Parent(parent_arr), depth + 1);
                }
            }
        }
    }

    fn finish(mut self) -> Vec<NodeType<M>> {
        self.reverse_tree.push(self.buffers.last().unwrap().0[0]);
        self.reverse_tree.reverse();
        let len = self.reverse_tree.len();
        self.reverse_tree.iter_mut().for_each(|n| {
            *n = match *n {
                NodeType::Empty => NodeType::Empty,
                NodeType::Leaf(val) => NodeType::Leaf(val),
                NodeType::Parent(mut val) => {
                    for v in &mut val {
                        if *v != 0 {
                            *v = len - *v;
                        }
                    }
                    NodeType::Parent(val)
                }
            }
        });
        self.reverse_tree
    }    
}

#[cfg(test)]
mod tests {
    use crate::morton;
    use rand::Rng;
    use super::*;

    #[test]
    fn tree_build() {
        let mut morton_list = Vec::new();
        let mut rng = rand::thread_rng();
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    let meta = rng.gen_range(0, 1);
                    morton_list.push(LeafMeta::new(meta, morton::encode(x, y, z)));
                }
            }
        }

        let tree = SparseVoxelOctree::build_from(morton_list);
        println!("{:?}", tree);
    }
}