// Right, Down, Left, Up
pub const DIRS: [(i64, i64); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

/// A node in a grid
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct Node {
    /// The node's x coordinate
    pub x: usize,

    /// The node's y coordinate
    pub y: usize,

    /// The index of the grid this node belongs to
    pub grid: usize,
}

/// An edge between two nodes
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Edge {
    /// The target node
    pub to: Node,

    /// The number of steps required to get from the source to the target node
    pub steps: usize,
}

/// A grid
pub struct Grid<T> {
    /// The actual grid
    pub grid: Vec<T>,

    /// The grid's width
    pub width: usize,

    /// The grid's height
    pub height: usize,
}

pub trait Get<T, C> {
    /// The the value at the given position
    fn get(&self, x: C, y: C) -> T;
}

impl<T> Get<T, usize> for Grid<T>
where
    T: Copy,
{
    fn get(&self, x: usize, y: usize) -> T {
        self.grid[y * self.width + x]
    }
}

impl<T> Get<T, i64> for Grid<T>
where
    T: Copy,
{
    fn get(&self, x: i64, y: i64) -> T {
        self.grid[y as usize * self.width + x as usize]
    }
}

pub trait Set<T, C> {
    /// Set the value at the given position
    fn set(&mut self, x: C, y: C, v: T);
}

impl<T> Set<T, usize> for Grid<T>
where
    T: Copy,
{
    fn set(&mut self, x: usize, y: usize, v: T) {
        self.grid[y * self.width + x] = v;
    }
}

impl<T> Set<T, i64> for Grid<T>
where
    T: Copy,
{
    fn set(&mut self, x: i64, y: i64, v: T) {
        self.grid[y as usize * self.width + x as usize] = v;
    }
}

pub trait Has<C> {
    /// Check if the given position is within the grid bounds
    fn has(&self, x: C, y: C) -> bool;
}

impl<T> Has<usize> for Grid<T> {
    fn has(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }
}

impl<T> Has<i64> for Grid<T> {
    fn has(&self, x: i64, y: i64) -> bool {
        x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height
    }
}
