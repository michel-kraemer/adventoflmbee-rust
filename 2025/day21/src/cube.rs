use rustc_hash::FxHashMap;

use crate::{Node, grid::Edge};

/// A direction on a cube side
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    Top = 0,
    Right = 1,
    Bottom = 2,
    Left = 3,
}

/// A canonical signature of a cube side edge
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Signature {
    width: usize,
    signature: Vec<usize>,
    nodes: Vec<Node>,
}

impl Signature {
    /// Create a new signature from the given nodes
    fn new(mut nodes: Vec<Node>, width: usize, height: usize) -> Self {
        if nodes.is_empty() {
            Self {
                width,
                signature: Vec::new(),
                nodes,
            }
        } else if nodes[0].x == 0 {
            // left edge
            nodes.sort_unstable_by_key(|n| height - n.y - 1);
            Self {
                width: height,
                signature: nodes.iter().map(|n| height - n.y - 1).collect(),
                nodes,
            }
        } else if nodes[0].x == width - 1 {
            // right edge
            nodes.sort_unstable_by_key(|n| n.y);
            Self {
                width: height,
                signature: nodes.iter().map(|n| n.y).collect(),
                nodes,
            }
        } else if nodes[0].y == 0 {
            // top edge
            nodes.sort_unstable_by_key(|n| n.x);
            Self {
                width,
                signature: nodes.iter().map(|n| n.x).collect(),
                nodes,
            }
        } else {
            // bottom edge
            nodes.sort_unstable_by_key(|n| width - n.x - 1);
            Self {
                width,
                signature: nodes.iter().map(|n| width - n.x - 1).collect(),
                nodes,
            }
        }
    }
}

impl Signature {
    /// Get matching nodes between this signature and another signature
    pub fn get_matches(&self, other: &Signature) -> Option<Vec<(Node, Node)>> {
        let mut self_iter = self.signature.iter().copied().enumerate();
        let (mut si, mut s) = self_iter.next()?;
        let mut other_iter = other
            .signature
            .iter()
            .map(|o| self.width - o - 1)
            .enumerate()
            .rev();
        let (mut oi, mut o) = other_iter.next()?;

        let mut result = Vec::new();
        'outer: loop {
            while s < o {
                if let Some((nexti, next)) = self_iter.next() {
                    si = nexti;
                    s = next;
                } else {
                    break 'outer;
                }
            }
            while o < s {
                if let Some((nexti, next)) = other_iter.next() {
                    oi = nexti;
                    o = next;
                } else {
                    break 'outer;
                }
            }
            if s == o {
                result.push((self.nodes[si], other.nodes[oi]));
                if let Some((nexti, next)) = self_iter.next() {
                    si = nexti;
                    s = next;
                } else {
                    break 'outer;
                }
            }
        }

        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }
}

/// A cube side
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Side {
    /// The signatures of the side's edges ordered by [Direction]
    signatures: [Signature; 4],
}

impl Side {
    /// Create a new side from the given graph
    pub fn new(graph: &FxHashMap<Node, Vec<Edge>>, width: usize, height: usize) -> Self {
        let left = graph
            .keys()
            .filter(|n| n.x == 0)
            .copied()
            .collect::<Vec<_>>();
        let right = graph
            .keys()
            .filter(|n| n.x == width - 1)
            .copied()
            .collect::<Vec<_>>();
        let top = graph
            .keys()
            .filter(|n| n.y == 0)
            .copied()
            .collect::<Vec<_>>();
        let bottom = graph
            .keys()
            .filter(|n| n.y == height - 1)
            .copied()
            .collect::<Vec<_>>();

        Self {
            signatures: [
                Signature::new(top, width, height),
                Signature::new(right, width, height),
                Signature::new(bottom, width, height),
                Signature::new(left, width, height),
            ],
        }
    }

    /// Get the signature of the edge in the given direction
    pub fn get_signature(&self, dir: Direction) -> &Signature {
        &self.signatures[dir as usize]
    }

    /// Rotate the side so that the edge in direction `from` becomes the edge in
    /// direction `to`
    pub fn to_rotated(&self, from: Direction, to: Direction) -> Self {
        let mut result = self.clone();
        result
            .signatures
            .rotate_right((to as isize - from as isize).rem_euclid(4) as usize);
        result
    }

    /// Get the neighbors of the side at the given index and in which directions
    /// they are connected.
    ///
    /// We represent the cube as follows (the numbers are cube side indexes):
    ///
    /// ```plain
    //     0
    //  2  1  3
    //     4
    //     5
    // ```
    //
    // The following sides (0-5) and directions (L,R,T,B) can be connected:
    //
    // ```plain
    // 0T-5B, 0R-3T, 0B-1T, 0L-2T
    // 1T-0B, 1R-3L, 1B-4T, 1L-2R
    // 2T-0L, 2R-1L, 2B-4L, 2L-5L
    // 3T-0R, 3R-5R, 3B-4R, 3L-1R
    // 4T-1B, 4R-3B, 4B-5T, 4L-2B
    // 5T-4B, 5R-3R, 5B-0T, 5L-2L
    // ```
    pub const fn get_neighbors(side: usize) -> [(Direction, usize, Direction); 4] {
        match side {
            // 0T-5B, 0R-3T, 0B-1T, 0L-2T
            0 => [
                (Direction::Top, 5, Direction::Bottom),
                (Direction::Right, 3, Direction::Top),
                (Direction::Bottom, 1, Direction::Top),
                (Direction::Left, 2, Direction::Top),
            ],
            // 1T-0B, 1R-3L, 1B-4T, 1L-2R
            1 => [
                (Direction::Top, 0, Direction::Bottom),
                (Direction::Right, 3, Direction::Left),
                (Direction::Bottom, 4, Direction::Top),
                (Direction::Left, 2, Direction::Right),
            ],
            // 2T-0L, 2R-1L, 2B-4L, 2L-5L
            2 => [
                (Direction::Top, 0, Direction::Left),
                (Direction::Right, 1, Direction::Left),
                (Direction::Bottom, 4, Direction::Left),
                (Direction::Left, 5, Direction::Left),
            ],
            // 3T-0R, 3R-5R, 3B-4R, 3L-1R
            3 => [
                (Direction::Top, 0, Direction::Right),
                (Direction::Right, 5, Direction::Right),
                (Direction::Bottom, 4, Direction::Right),
                (Direction::Left, 1, Direction::Right),
            ],
            // 4T-1B, 4R-3B, 4B-5T, 4L-2B
            4 => [
                (Direction::Top, 1, Direction::Bottom),
                (Direction::Right, 3, Direction::Bottom),
                (Direction::Bottom, 5, Direction::Top),
                (Direction::Left, 2, Direction::Bottom),
            ],
            // 5T-4B, 5R-3R, 5B-0T, 5L-2L
            _ => [
                (Direction::Top, 4, Direction::Bottom),
                (Direction::Right, 3, Direction::Right),
                (Direction::Bottom, 0, Direction::Top),
                (Direction::Left, 2, Direction::Left),
            ],
        }
    }
}
