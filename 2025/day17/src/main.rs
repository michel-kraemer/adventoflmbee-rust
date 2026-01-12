use std::fs;

use rustc_hash::{FxHashMap, FxHashSet};

fn longest_chain(
    start: u64,
    graph: &FxHashMap<u64, Vec<u64>>,
    cache: &mut FxHashMap<u64, u64>,
) -> u64 {
    if let Some(c) = cache.get(&start) {
        return *c;
    }
    let mut result = 1;
    if let Some(neighbors) = graph.get(&start) {
        for n in neighbors {
            result = result.max(1 + longest_chain(*n, graph, cache));
        }
    }
    cache.insert(start, result);
    result
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let mut graph: FxHashMap<u64, Vec<u64>> = FxHashMap::default();
    let mut all_nodes = FxHashSet::default();
    let mut destinations = FxHashSet::default();
    for l in input.lines() {
        let (from, to) = l.split_once(" -> ").unwrap();
        let from = from.parse::<u64>().unwrap();
        let to = to.parse::<u64>().unwrap();
        graph.entry(from).or_default().push(to);
        all_nodes.insert(from);
        all_nodes.insert(to);
        destinations.insert(to);
    }

    // part 1
    let mut cache = FxHashMap::default();
    println!(
        "{}",
        graph
            .keys()
            .map(|from| longest_chain(*from, &graph, &mut cache))
            .max()
            .unwrap()
    );

    // part 2 - the number of additional edges required to make a graph strongly
    // connected is max(N - O, N - I) where N is the total number of nodes, O is
    // the number of nodes that have outgoing edges, and I is the number of
    // nodes having incoming edges.
    //
    // Note that the problem statement contains an error: The edges given in
    // part 2 to make the graph strongly connected never connect to 0, so no
    // node can ever reach 0! One possible way to make the graph in part 2
    // strongly connected would instead be: 6->4, 7->0, 5->1
    println!(
        "{}",
        (all_nodes.len() - graph.len()).max(all_nodes.len() - destinations.len())
    );
}
