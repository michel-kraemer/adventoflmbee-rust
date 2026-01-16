use std::{cmp::Ordering, collections::BinaryHeap, fs};

use rayon::prelude::*;

// Right, Down, Left, Up
pub const DIRS: [(i64, i64); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Node {
    x: i64,
    y: i64,
    proxy: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Edge {
    to: Node,
    weight: i64,
}

#[derive(Clone)]
struct Graph<T> {
    graph: Vec<T>,
    width: i64,
    height: i64,
}

impl<T> Graph<T>
where
    T: Default + Clone,
{
    fn new(width: i64, height: i64) -> Self {
        Self {
            graph: vec![Default::default(); width as usize * height as usize * 2],
            width,
            height,
        }
    }

    fn insert(&mut self, from: Node, to: T) {
        self.graph[(from.proxy as i64 * self.height * self.width + from.y * self.width + from.x)
            as usize] = to;
    }

    fn get(&self, n: Node) -> &T {
        &self.graph[(n.proxy as i64 * self.height * self.width + n.y * self.width + n.x) as usize]
    }

    fn get_mut(&mut self, n: Node) -> &mut T {
        &mut self.graph
            [(n.proxy as i64 * self.height * self.width + n.y * self.width + n.x) as usize]
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct State {
    cost: i64,
    pos: Node,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Perform Dijkstra's algorithm to find the shortest path in a graph between
/// the given start and end nodes. Returns a map of the shortest distances
/// between the start node and all graph nodes, as well as the found shortest
/// path.
fn dijkstra(
    graph: &Graph<Vec<Edge>>,
    start: Node,
    end: Node,
) -> (Graph<Option<i64>>, Vec<(Node, Node)>) {
    let mut queue = BinaryHeap::new();
    queue.push(State {
        cost: 0,
        pos: start,
    });

    let mut best: Graph<Option<i64>> = Graph::new(graph.width, graph.height);
    let mut parent: Graph<Node> = Graph::new(graph.width, graph.height);
    best.insert(start, Some(0));

    while let Some(State { cost, pos }) = queue.pop() {
        let neighbors = graph.get(pos);
        for &n in neighbors {
            let new_cost = cost + n.weight;
            let old_best = best.get(n.to).unwrap_or(i64::MAX);
            if old_best > new_cost {
                best.insert(n.to, Some(new_cost));
                parent.insert(n.to, pos);
                queue.push(State {
                    cost: new_cost,
                    pos: n.to,
                });
            }
        }
    }

    let mut path = Vec::new();
    let mut current = end;
    while current != start {
        let next = *parent.get(current);
        path.push((next, current));
        current = next;
    }
    path.reverse();

    (best, path)
}

fn main() {
    // We can solve both parts simultaneously. For part 2, we use Suurballe's
    // algorithm (https://en.wikipedia.org/wiki/Suurballe%27s_algorithm), to
    // find two vertex-disjoint paths from the start node S to the end node E
    // with minimum total length. The Wikipedia page describes the algorithm in
    // a variant where it only finds edge-disjoint paths. We therefore adapt the
    // approach from the paper "Survivable Node-Disjoint Routing in Multi-Domain
    // Networks" by Samonaki et al.
    // (https://ieeexplore.ieee.org/document/10278855/, DOI
    // 10.1109/ICC45041.2023.10278855) and split the nodes along the shortest
    // path found in the first step of the algorithm into two nodes, one with
    // incoming edges and one with outgoing edges.
    //
    // To solve part 1, we can use the results of the first step of the
    // algorithm (Dijkstra).

    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let blocks = input.split("\n\n");

    let (sums1, sums2): (Vec<i64>, Vec<i64>) = blocks
        .par_bridge()
        .map(|b| {
            // parse grid
            let lines = b.lines().collect::<Vec<_>>();
            let width = lines[0].len() as i64;
            let height = lines.len() as i64;
            let grid = lines
                .into_iter()
                .flat_map(|l| {
                    l.bytes().map(|b| {
                        if b.is_ascii_digit() {
                            (b - b'0') as i64
                        } else {
                            0
                        }
                    })
                })
                .collect::<Vec<_>>();

            // convert grid to graph
            let mut graph: Graph<Vec<Edge>> = Graph::new(width, height);
            for y in 0..height {
                for x in 0..width {
                    for (dx, dy) in DIRS {
                        let nx = x + dx;
                        let ny = y + dy;
                        if nx >= 0 && ny >= 0 && nx < width && ny < height {
                            let weight = grid[(ny * width + nx) as usize];
                            graph.get_mut(Node { x, y, proxy: false }).push(Edge {
                                to: Node {
                                    x: nx,
                                    y: ny,
                                    proxy: false,
                                },
                                weight,
                            });
                        }
                    }
                }
            }

            // step 1: perform Dijkstra's algorithm to find the shortest distances
            // between the start node and all graph nodes, as well as the shortest
            // path to the end node.
            let start = Node {
                x: 0,
                y: 0,
                proxy: false,
            };
            let end = Node {
                x: width - 1,
                y: height - 1,
                proxy: false,
            };
            let (best, mut path) = dijkstra(&graph, start, end);

            // the result of step 1 can be used to solve part 1
            let sum1 = best.get(end).unwrap();

            // step 2: create residual graph (i.e. modify edge weights)
            let mut residual_graph = graph.clone();
            for y in 0..height {
                for x in 0..width {
                    let from = Node { x, y, proxy: false };
                    let edges = residual_graph.get_mut(from);
                    for n in edges {
                        n.weight = n.weight - best.get(n.to).unwrap() + best.get(from).unwrap();
                    }
                }
            }

            // traverse the found shortest path backwards
            for &(from, to) in path.iter().rev() {
                // remove all edges along the path
                let e = residual_graph.get_mut(from);
                e.swap_remove(e.iter().position(|n| n.to == to).unwrap());
                let f = residual_graph.get_mut(to);
                f.swap_remove(f.iter().position(|n| n.to == from).unwrap());

                if to != end {
                    // Split all nodes along the path that are not the start node
                    // and not the end node into two nodes: one that only has
                    // incoming edges and a proxy node that only has outgoing edges.
                    // This is necessary so the algorithm finds two node-disjoint
                    // paths and not only edge-disjoint paths. Also, insert an edge
                    // from the proxy node to the original node with weight 0.
                    let mut outgoing = std::mem::take(f);
                    outgoing.push(Edge { to, weight: 0 });
                    residual_graph.insert(
                        Node {
                            x: to.x,
                            y: to.y,
                            proxy: true,
                        },
                        outgoing,
                    );
                }

                // reinsert edges with weight 0 from the end node to the start node
                let f = residual_graph.get_mut(to);
                if from == start {
                    f.push(Edge {
                        to: from,
                        weight: 0,
                    });
                } else {
                    f.push(Edge {
                        to: Node {
                            x: from.x,
                            y: from.y,
                            proxy: true,
                        },
                        weight: 0,
                    });
                }
            }

            // step 3: perform Dijkstra's again on the modified graph. This results
            // in another shortest path
            let (_, mut path2) = dijkstra(&residual_graph, start, end);

            // remove proxy nodes from the second shortest path and remove cycles
            // with zero length
            for (from, to) in &mut path2 {
                from.proxy = false;
                to.proxy = false;
            }
            path2.retain(|(from, to)| from != to);

            // step 4: iterate through the second path, and whenever we find the
            // reverse of an edge in the first path, remove the edge from the second
            // path and its reverse from the first
            let mut i = 0;
            while i < path2.len() {
                let (from, to) = path2[i];
                if let Some(j) = path.iter().position(|n| n.0 == to && n.1 == from) {
                    path.swap_remove(j);
                    path2.swap_remove(i);
                } else {
                    i += 1;
                }
            }

            // step 5 would actually be to reconstruct the two disjoint paths. This
            // is not necessary as we are only interested in the sum of the edge
            // weights.
            let mut sum2 = 0;
            for (from, to) in path {
                sum2 += graph.get(from).iter().find(|n| n.to == to).unwrap().weight;
            }
            for (from, to) in path2 {
                sum2 += graph.get(from).iter().find(|n| n.to == to).unwrap().weight;
            }

            (sum1, sum2)
        })
        .unzip();

    println!("{}", sums1.into_iter().product::<i64>());
    println!("{}", sums2.into_iter().product::<i64>());
}
