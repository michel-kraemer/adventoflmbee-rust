use std::fs;

use dashu_int::UBig;
use rustc_hash::FxHashMap;

const INP_INDEX: usize = 27 * 27 * 27 - 3;
const OUT_INDEX: usize = 27 * 27 * 27 - 2;
const BIN_INDEX: usize = 27 * 27 * 27 - 1;

fn index(name: &str) -> usize {
    if name == "INP" {
        INP_INDEX
    } else if name == "OUT" {
        OUT_INDEX
    } else if name == "BIN" {
        BIN_INDEX
    } else {
        let b = name.as_bytes();
        (b[0] - b'a') as usize * 27 * 27 + (b[1] - b'a') as usize * 27 + (b[2] - b'a') as usize
    }
}

fn count(
    s: usize,
    map: &[Vec<usize>],
    n_nodes: usize,
    path_len: usize,
    cache: &mut FxHashMap<(usize, usize), UBig>,
) -> UBig {
    if s == OUT_INDEX {
        // we found a path to OUT - multiply it with the number of remaining
        // system states to determine how often this path will appear
        return UBig::from(2u64).pow(n_nodes - path_len);
    }
    if s == BIN_INDEX {
        return UBig::ZERO;
    }

    if let Some(c) = cache.get(&(s, path_len)) {
        return c.clone();
    }

    let mut result = UBig::ZERO;
    for &n in &map[s] {
        result += count(n, map, n_nodes, path_len + 1, cache);
    }

    cache.insert((s, path_len), result.clone());

    result
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let mut map = vec![Vec::new(); 27 * 27 * 27];
    let mut n_nodes = 0;
    for l in input.lines() {
        n_nodes += 1;
        let (from, to) = l.split_once(": ").unwrap();
        for t in to.split_ascii_whitespace() {
            map[index(from)].push(index(t));
        }
    }

    // part 1
    let mut states = vec![0; 27 * 27 * 27];
    let mut total1 = 0;
    for _ in 0..123456 {
        let mut pos = INP_INDEX;
        while pos != OUT_INDEX && pos != BIN_INDEX {
            let next = map[pos][states[pos]];
            states[pos] = (states[pos] + 1) % map[pos].len();
            pos = next;
        }
        if pos == OUT_INDEX {
            total1 += 1;
        }
    }
    println!("{total1}");

    // part 2
    let n_signals = UBig::from(12u64).pow(3456);

    // there are 2^n possible states of the whole system, where n is the number
    // of binary flip-flops (note that the first entry in the input is not a
    // flip-flop)
    let total_states = UBig::from(2u64).pow(n_nodes - 1);

    // The input conveniently makes sure that 12^3456 is divisible by the number
    // of states. This allows us to use a simple formula to calculate the
    // answer.
    assert!(n_signals.is_multiple_of(&total_states));

    let total2 = (n_signals / total_states
        * count(INP_INDEX, &map, n_nodes, 0, &mut FxHashMap::default()))
        % 1_000_000_000_000_000u64;
    println!("{total2}");
}
