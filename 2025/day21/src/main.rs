use std::{collections::VecDeque, fs};

use rustc_hash::{FxHashMap, FxHashSet};

use crate::{
    cube::{Direction, Side},
    grid::{DIRS, Edge, Get, Grid, Has, Node, Set as _},
    unionfind::{Set, find, union},
};

mod cube;
mod grid;
mod unionfind;

/// Converts a grid to a graph where every outlet, junction, and grid exit is a
/// node and the nodes are all connected through edges
fn convert_to_graph(grid: &Grid<u8>, gi: usize, graph: &mut FxHashMap<Node, Vec<Edge>>) {
    fn dfs(
        pos: Node,
        grid: &Grid<u8>,
        seen: &mut Grid<bool>,
        mut steps: usize,
        graph: &mut FxHashMap<Node, Vec<Edge>>,
        mut start: Node,
    ) {
        seen.set(pos.x, pos.y, true);

        let c = grid.get(pos.x, pos.y);
        if steps > 0
            && (c == b'O'
                || (c == b'.'
                    && (pos.x == 0
                        || pos.y == 0
                        || pos.x == grid.width - 1
                        || pos.y == grid.height - 1)))
        {
            // we've stepped on an outlet or an exit at the grid's border - add
            // a new edge from our current start node to here
            graph
                .entry(start)
                .or_default()
                .push(Edge { to: pos, steps });
            graph
                .entry(pos)
                .or_default()
                .push(Edge { to: start, steps });
            start = pos;
            steps = 0;
        }

        // find all neighbors of the current position
        let mut neighbors = Vec::new();
        for (dx, dy) in DIRS {
            let nx = pos.x as i64 + dx;
            let ny = pos.y as i64 + dy;
            if grid.has(nx, ny) && !seen.get(nx, ny) && grid.get(nx, ny) != b'#' {
                neighbors.push((nx as usize, ny as usize));
            }
        }

        if steps > 0 && neighbors.len() > 1 {
            // we're at a junction - add a new edge from the current start node
            // to here
            graph
                .entry(start)
                .or_default()
                .push(Edge { to: pos, steps });
            graph
                .entry(pos)
                .or_default()
                .push(Edge { to: start, steps });
            start = pos;
            steps = 0;
        }

        // continue with all neighbors
        for n in neighbors {
            dfs(
                Node {
                    x: n.0,
                    y: n.1,
                    grid: start.grid,
                },
                grid,
                seen,
                steps + 1,
                graph,
                start,
            );
        }
    }

    // find all start points
    let mut start_points = Vec::new();
    for y in 0..grid.height {
        for x in 0..grid.width {
            let c = grid.get(x, y);
            if c == b'O'
                || (c == b'.' && (x == 0 || y == 0 || x == grid.width - 1 || y == grid.height - 1))
            {
                start_points.push(Node { x, y, grid: gi });
            }
        }
    }

    // perform DFS to connect all outlets, junctions, and grid exits
    let mut seen = Grid {
        grid: vec![false; grid.width * grid.height],
        width: grid.width,
        height: grid.height,
    };
    for s in start_points {
        dfs(s, grid, &mut seen, 0, graph, s);
    }
}

/// "Compress" a graph (i.e. remove all nodes that only have two neighbors and
/// connect the neighbors directly)
fn compress_graph(graph: &mut FxHashMap<Node, Vec<Edge>>, grids: &[Grid<u8>]) {
    let nodes = graph
        .iter()
        .filter(|(node, edges)| edges.len() == 2 && grids[node.grid].get(node.x, node.y) != b'O')
        .map(|(node, _)| node)
        .copied()
        .collect::<Vec<_>>();

    for node in nodes {
        let edges = graph.remove(&node).unwrap();

        let v0 = graph.get_mut(&edges[0].to).unwrap();
        v0.remove(v0.iter().position(|o| o.to == node).unwrap());
        v0.push(Edge {
            to: edges[1].to,
            steps: edges[0].steps + edges[1].steps,
        });

        let v1 = graph.get_mut(&edges[1].to).unwrap();
        v1.remove(v1.iter().position(|o| o.to == node).unwrap());
        v1.push(Edge {
            to: edges[0].to,
            steps: edges[0].steps + edges[1].steps,
        });
    }
}

/// Remove nodes with only one neighbor and which are not outlets
fn clean_graph(full_graph: &mut FxHashMap<Node, Vec<Edge>>, grids: &[Grid<u8>]) {
    let mut queue = full_graph
        .iter()
        .filter(|(node, edges)| edges.len() == 1 && grids[node.grid].get(node.x, node.y) != b'O')
        .map(|(node, edges)| (*node, edges.clone()))
        .collect::<Vec<_>>();
    while let Some((node, edges)) = queue.pop() {
        let v0 = full_graph.get_mut(&edges[0].to).unwrap();
        v0.remove(v0.iter().position(|o| o.to == node).unwrap());
        if v0.is_empty() {
            full_graph.remove(&edges[0].to);
        } else if v0.len() == 1 {
            queue.push((edges[0].to, v0.clone()));
        }
        full_graph.remove(&node);
    }
}

/// Find all possible configurations in which we can put graphs on each side of
/// the cube and rotate them so they are connected. We assume that all sides are
/// somehow connected. Otherwise, the problem would not be solvable, so this
/// assumption is safe. However, this also means that it won't work for the
/// example (Which actually does not match the problem statement saying "On each
/// square grid, there are two outlet 'O's". The example contains square grids
/// without any outlet.).
fn get_cube_configurations(
    sides: &[Side],
    graphs: &[FxHashMap<Node, Vec<Edge>>],
    grids: &[Grid<u8>],
) -> Vec<Vec<Side>> {
    let mut queue: VecDeque<(Vec<Option<Side>>, u8)> = VecDeque::new();
    queue.push_back((
        vec![
            None,
            Some(Side::new(&graphs[0], grids[0].width, grids[0].height)),
            None,
            None,
            None,
            None,
        ],
        0b000001,
    ));

    // perform BFS
    let mut valid_configurations = Vec::new();
    let mut seen = FxHashSet::default();
    while let Some((configuration, used)) = queue.pop_front() {
        if configuration.iter().all(|s| s.is_some()) {
            valid_configurations.push(configuration);
            continue;
        }

        // for every side in the configuration that has already been assigned ...
        for (i, s) in configuration.iter().enumerate() {
            let Some(s) = s else {
                continue;
            };

            // for every neighbor of the current side that is still free ...
            let neighbors = Side::get_neighbors(i);
            for (from_dir, to_side, to_dir) in neighbors {
                if configuration[to_side].is_some() {
                    continue;
                }

                // try to find a graph that has not been used yet and that can
                // can be assigned to this cube's side
                for (oi, other) in sides.iter().enumerate() {
                    if used & (1 << oi) > 0 {
                        continue;
                    }

                    // check if any edge signature matches
                    let ss = s.get_signature(from_dir);
                    for od in [
                        Direction::Top,
                        Direction::Right,
                        Direction::Bottom,
                        Direction::Left,
                    ] {
                        let os = sides[oi].get_signature(od);
                        if ss.get_matches(os).is_some() {
                            // rotate the graph and assign it to the configuration
                            let new_side = other.to_rotated(od, to_dir);
                            let mut new_configuration = configuration.clone();
                            new_configuration[to_side] = Some(new_side);
                            let new_entry = (new_configuration, used | (1 << oi));
                            if seen.insert(new_entry.clone()) {
                                queue.push_back(new_entry);
                            }
                        }
                    }
                }
            }
        }
    }

    // valid_configurations only contains elements where all Options have values
    valid_configurations
        .into_iter()
        .map(|c| c.into_iter().map(|s| s.unwrap()).collect())
        .collect()
}

/// Find all possible connections between every pair of cube sides
fn get_connections(configuration: &[Side]) -> Vec<(Node, Node)> {
    let mut result = Vec::new();
    for (i, s) in configuration.iter().enumerate() {
        let neighbors = Side::get_neighbors(i);
        for (from_dir, to_side, to_dir) in neighbors {
            if let Some(matches) = s
                .get_signature(from_dir)
                .get_matches(configuration[to_side].get_signature(to_dir))
            {
                result.extend(matches);
            }
        }
    }
    result
}

/// Find a single minimum spanning tree in the given graph. Return `None` if
/// there are more than one tree. Otherwise, return the sum of the edges of the
/// tree.
fn single_mst(graph: FxHashMap<Node, Vec<Edge>>) -> Option<usize> {
    // collect all unique edges
    let mut unique_edges = Vec::new();
    for (&from, v) in &graph {
        for u in v {
            let mut from = from;
            let mut to = u.to;
            let dist = u.steps;
            if to < from {
                (to, from) = (from, to);
            }
            unique_edges.push((from, to, dist));
        }
    }
    unique_edges.sort_unstable();
    unique_edges.dedup();

    // sort edges by increasing number of steps
    unique_edges.sort_unstable_by_key(|e| e.2);

    // create sets for union-find data structure
    let mut sets = graph
        .keys()
        .enumerate()
        .map(|k| Set {
            node: *k.1,
            parent: k.0,
            size: 1,
        })
        .collect::<Vec<_>>();

    // create minimum spanning tree(s)
    let mut total = 0;
    for (u, v, dist) in &unique_edges {
        let ui = find(sets.iter().position(|n| n.node == *u).unwrap(), &mut sets);
        let vi = find(sets.iter().position(|n| n.node == *v).unwrap(), &mut sets);
        if ui != vi {
            total += dist;
            union(ui, vi, &mut sets);
        }
    }

    // check if there's only one MST
    let mut parents = FxHashSet::default();
    for i in 0..sets.len() {
        let s = find(i, &mut sets);
        parents.insert(s);
        if parents.len() > 1 {
            // there is more than one MST
            return None;
        }
    }

    Some(total)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let grids = input
        .split("\n\n")
        .map(|b| {
            let lines = b.lines().collect::<Vec<_>>();
            let width = lines[0].len();
            let height = lines.len();
            let grid = lines
                .into_iter()
                .flat_map(|l| l.bytes())
                .collect::<Vec<_>>();
            Grid {
                grid,
                width,
                height,
            }
        })
        .collect::<Vec<_>>();

    // part 1 - simple BFS
    let mut total1 = 1;
    for grid in &grids {
        let mut start = (0, 0);
        'outer: for y in 0..grid.height {
            for x in 0..grid.width {
                if grid.get(x, y) == b'O' {
                    start = (x, y);
                    break 'outer;
                }
            }
        }

        let mut queue = VecDeque::new();
        queue.push_back((start.0, start.1, 0));
        let mut seen = Grid {
            grid: vec![u64::MAX; grid.width * grid.height],
            width: grid.width,
            height: grid.height,
        };
        seen.set(start.0, start.1, 0);

        while let Some((x, y, steps)) = queue.pop_front() {
            if (x, y) != start && grid.get(x, y) == b'O' {
                total1 *= steps - 1;
                break;
            }
            for (dx, dy) in DIRS {
                let nx = x as i64 + dx;
                let ny = y as i64 + dy;
                if grid.has(nx, ny) && seen.get(nx, ny) > steps + 1 && grid.get(nx, ny) != b'#' {
                    seen.set(nx, ny, steps + 1);
                    queue.push_back((nx as usize, ny as usize, steps + 1));
                }
            }
        }
    }
    println!("{total1}");

    // part 2 ...

    // convert all grids to graphs
    let mut graphs = Vec::new();
    for (gi, grid) in grids.iter().enumerate() {
        let mut graph = FxHashMap::default();
        convert_to_graph(grid, gi, &mut graph);

        // remove unnecessary nodes
        compress_graph(&mut graph, &grids);

        graphs.push(graph);
    }

    // convert graphs to cube sides
    let sides = graphs
        .iter()
        .map(|g| Side::new(g, grids[0].width, grids[0].height))
        .collect::<Vec<_>>();

    // find valid cube configurations
    let valid_configurations = get_cube_configurations(&sides, &graphs, &grids);

    let mut min = usize::MAX;
    for configuration in valid_configurations {
        // put every graph into a new full graph
        let mut full_graph: FxHashMap<Node, Vec<Edge>> = FxHashMap::default();
        for g in &graphs {
            full_graph.extend(g.iter().map(|(k, v)| (*k, v.clone())));
        }

        // find all valid connections between cube sides
        let connections = get_connections(&configuration);

        // add these connections to the full graph
        for connection in connections {
            full_graph.entry(connection.0).or_default().push(Edge {
                to: connection.1,
                steps: 1,
            });
        }

        // Recursively remove nodes that have only one neighbor and that are not
        // outlets. This also removes any connected component that does not
        // contain an outlet.
        clean_graph(&mut full_graph, &grids);

        // compress the graph again to remove nodes that have only two neighbors
        // and that are not outlets
        compress_graph(&mut full_graph, &grids);

        if let Some(steps) = single_mst(full_graph) {
            // What we get from the MST is the number of steps required to get
            // from a starting outlet to any other outlet, but we need the
            // number of grid cells we have to fill. Add 1 for the starting
            // outlet and then subtract the total number of outlets.
            min = min.min((steps + 1) - grids.len() * 2);
        }
    }
    println!("{min}");
}
