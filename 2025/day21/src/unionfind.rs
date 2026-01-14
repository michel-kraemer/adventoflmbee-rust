use crate::grid::Node;

#[derive(Debug)]
pub struct Set {
    pub node: Node,
    pub parent: usize,
    pub size: usize,
}

pub fn find(x: usize, sets: &mut [Set]) -> usize {
    if sets[x].parent != x {
        sets[x].parent = find(sets[x].parent, sets);
        sets[x].parent
    } else {
        x
    }
}

pub fn union(x: usize, y: usize, sets: &mut [Set]) {
    let mut x = find(x, sets);
    let mut y = find(y, sets);

    if x == y {
        return;
    }

    if sets[x].size < sets[y].size {
        (x, y) = (y, x);
    }

    sets[y].parent = x;
    sets[x].size += sets[y].size;
}
