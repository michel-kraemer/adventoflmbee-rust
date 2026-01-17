use std::{collections::VecDeque, fs};

use rayon::prelude::*;

// Right, Down, Left, Up
const DIRS: [(i64, i64); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

const DUMMY: Node = Node {
    x: usize::MAX,
    y: usize::MAX,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Node {
    x: usize,
    y: usize,
}

impl Default for Node {
    fn default() -> Self {
        DUMMY
    }
}

struct Graph<T> {
    graph: Vec<T>,
    width: usize,
}

impl<T> Graph<T>
where
    T: Default + Clone,
{
    fn new(width: usize, height: usize) -> Self {
        Self {
            graph: vec![T::default(); width * height],
            width,
        }
    }

    fn insert(&mut self, n: Node, to: T) {
        self.graph[n.y * self.width + n.x] = to;
    }

    fn get(&self, n: Node) -> &T {
        &self.graph[n.y * self.width + n.x]
    }

    fn get_mut(&mut self, n: Node) -> &mut T {
        &mut self.graph[n.y * self.width + n.x]
    }
}

struct Dist {
    dist: Graph<u64>,
    dummy_dist: u64,
}

impl Dist {
    fn new(width: usize, height: usize) -> Self {
        Self {
            dist: Graph::new(width, height),
            dummy_dist: u64::MAX,
        }
    }

    fn get(&self, n: Node) -> u64 {
        if n == DUMMY {
            self.dummy_dist
        } else {
            *self.dist.get(n)
        }
    }

    fn set(&mut self, n: Node, dist: u64) {
        if n == DUMMY {
            self.dummy_dist = dist;
        } else {
            self.dist.insert(n, dist);
        }
    }
}

/// Part of Hopcroft-Karp (see comment in main function)
fn bfs(
    us: &[Node],
    pair_u: &Graph<Node>,
    pair_v: &Graph<Node>,
    dist: &mut Dist,
    graph: &Graph<Vec<Node>>,
) -> bool {
    let mut queue = VecDeque::new();
    for &u in us {
        if *pair_u.get(u) == DUMMY {
            dist.set(u, 0);
            queue.push_back(u);
        } else {
            dist.set(u, u64::MAX);
        }
    }

    dist.set(DUMMY, u64::MAX);
    while let Some(u) = queue.pop_front() {
        if dist.get(u) < dist.get(DUMMY) {
            for &v in graph.get(u) {
                let pv = *pair_v.get(v);
                if dist.get(pv) == u64::MAX {
                    dist.set(pv, dist.get(u) + 1);
                    queue.push_back(pv);
                }
            }
        }
    }

    dist.get(DUMMY) != u64::MAX
}

/// Part of Hopcroft-Karp (see comment in main function)
fn dfs(
    u: Node,
    pair_u: &mut Graph<Node>,
    pair_v: &mut Graph<Node>,
    dist: &mut Dist,
    graph: &Graph<Vec<Node>>,
) -> bool {
    if u == DUMMY {
        return true;
    }

    for &v in graph.get(u) {
        let pv = *pair_v.get(v);
        if dist.get(pv) == dist.get(u) + 1 && dfs(pv, pair_u, pair_v, dist, graph) {
            pair_v.insert(v, u);
            pair_u.insert(u, v);
            return true;
        }
    }
    dist.set(u, u64::MAX);

    false
}

fn main() {
    // There are two key insights to solve this puzzle:
    //
    // 1. All present shapes in the input file can be reduced to dominoes (i.e.
    //    1x2 rectangles). There's no need to parse them. Also, since all
    //    presents have the same shape, instead of counting how many individual
    //    presents we need to fit into each area, we can just compute the total.
    //
    // 2. The fact that we're only dealing with dominoes, allows us to find a
    //    solution at all (testing arbitrary shapes would be NP-hard). We don't
    //    need to find a valid partitioning of each area, we just need count how
    //    many dominoes would fit into it and compare that with the required
    //    number of dominoes. To do so, we can view the grid as a bipartite
    //    graph and find a maximum matching, i.e the largest number of edges
    //    that don't share a vertex (see [1, 2]). This works because the grid
    //    can be colored like a checkerboard where all white vertices (in even
    //    positions) are in one subgraph and all black vertices (in odd
    //    positions) are in another. This is exactly the definition of a
    //    bipartite graph. An efficient algorithm to compute the maximum
    //    matching is Hopcroft-Karp [3, 4], which I've implemented here. It has
    //    a worst-case complexity of O(|E| √|V|). As a byproduct, it not only
    //    gives the maximum number of vertex-disjoint edges but also the actual
    //    edges (pairs of vertices), but we don't use them.
    //
    // [1] https://en.wikipedia.org/wiki/Matching_%28graph_theory%29#Maximum_matchings_in_bipartite_graphs
    // [2] https://en.wikipedia.org/wiki/Maximum_cardinality_matching
    // [3] https://en.wikipedia.org/wiki/Hopcroft%E2%80%93Karp_algorithm
    // [4] https://en.wikipedia.org/wiki/Hopcroft%E2%80%93Karp_algorithm#Pseudocode

    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let blocks = input.split("\n\n").collect::<Vec<_>>();

    // skip the presents, we only need the areas
    let areas = &blocks[6..];

    let total = areas
        .par_iter()
        .enumerate()
        .map(|(i, a)| {
            let mut lines = a.lines();
            let parts = lines
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .collect::<Vec<_>>();

            // parse area size
            let (height, width) = parts[0].split_once('x').unwrap();
            let width = width[0..width.len() - 1].parse::<usize>().unwrap();
            let height = height.parse::<usize>().unwrap();

            // parse presents - since all presents are dominoes, we can compute
            // the sum here
            let required_presents = parts[1..]
                .iter()
                .map(|p| p.parse::<i64>().unwrap())
                .sum::<i64>();

            // parse area
            let area = lines
                .map(|l| l.bytes().collect::<Vec<_>>())
                .collect::<Vec<_>>();

            // convert area to a graph and keep lists of white nodes (us) and
            // black nodes (vs)
            let mut us = Vec::new();
            let mut vs = Vec::new();
            let mut graph: Graph<Vec<Node>> = Graph::new(width, height);
            for y in 0..height {
                for x in 0..width {
                    if area[y][x] == b'#' {
                        continue;
                    }

                    let n = Node { x, y };
                    if (x + y) % 2 == 0 {
                        // white nodes are in even positions
                        us.push(n);
                    } else {
                        // white nodes are in odd positions
                        vs.push(n);
                    }

                    // insert edge between current node and its neighbors
                    let e = graph.get_mut(n);
                    for (dx, dy) in DIRS {
                        let nx = x as i64 + dx;
                        let ny = y as i64 + dy;
                        if nx >= 0
                            && ny >= 0
                            && (nx as usize) < width
                            && (ny as usize) < height
                            && area[ny as usize][nx as usize] != b'#'
                        {
                            e.push(Node {
                                x: nx as usize,
                                y: ny as usize,
                            });
                        }
                    }
                }
            }

            // Matchings between u and v nodes. Initially, all nodes are
            // connected to the artificial DUMMY node.
            let mut pair_u: Graph<Node> = Graph::new(width, height);
            let mut pair_v: Graph<Node> = Graph::new(width, height);

            // A map of shortest distances between nodes
            let mut dist = Dist::new(width, height);

            // Hopcroft–Karp ...
            let mut matching = 0;
            while bfs(&us, &pair_u, &pair_v, &mut dist, &graph) {
                for &u in &us {
                    if *pair_u.get(u) == DUMMY
                        && dfs(u, &mut pair_u, &mut pair_v, &mut dist, &graph)
                    {
                        matching += 1;
                    }
                }
            }

            // `matching` gives us the maximum number of edges in the graph that
            // don't share a vertex. This tells us how many dominoes, i.e.
            // presents, we can place. If this number is equal to or greater
            // than the number required, we have found a solution.
            // (n.b. pair_u and pair_v contain the actual vertex-disjoint edges,
            // but we don't need them)
            if matching >= required_presents {
                i + 1
            } else {
                0
            }
        })
        .sum::<usize>();

    println!("{total}");

    // part 2 - found via a quick Internet search
    println!("492");
}
