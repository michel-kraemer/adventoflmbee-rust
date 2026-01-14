use std::fs;
use std::ops::RangeInclusive;

fn count_digit(n: u64, digit: u64) -> u32 {
    let mut result = 0;
    let mut m = n;
    while m > 0 {
        let d = m % 10;
        m /= 10;
        if d == digit {
            result += 1;
        }
    }
    result
}

fn dfs(
    pos: u64,
    count: u64,
    remainder: u64,
    favorite_digit: u64,
    favorite_number: u64,
    len: &RangeInclusive<u64>,
    cache: &mut [u64],
) -> u64 {
    let ok = pos >= *len.start() && remainder == 0 && count >= pos.div_ceil(2);

    if pos == *len.end() {
        if ok {
            return 1;
        }
        return 0;
    }

    let cache_index =
        (pos * len.end() * favorite_number + count * favorite_number + remainder) as usize;
    if cache[cache_index] != u64::MAX {
        return cache[cache_index];
    }

    let mut result = if ok { 1 } else { 0 };
    for d in if pos > 0 { 0..=9 } else { 1..=9 } {
        let nc = if d == favorite_digit {
            count + 1
        } else {
            count
        };

        let nr = (remainder * 10 + d) % favorite_number;

        result += dfs(pos + 1, nc, nr, favorite_digit, favorite_number, len, cache);
    }

    cache[cache_index] = result;

    result
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let blocks = input.split("\n\n");

    let mut total1 = 0;
    let mut total2 = 0;
    for block in blocks {
        let mut l = block.lines().skip(1);
        let (_, favorite_digit) = l.next().unwrap().rsplit_once(' ').unwrap();
        let (_, favorite_number) = l.next().unwrap().rsplit_once(' ').unwrap();
        let favorite_digit = favorite_digit.parse::<u64>().unwrap();
        let favorite_number = favorite_number.parse::<u64>().unwrap();

        let mut n = 0;
        loop {
            n += favorite_number;
            let len = n.ilog10() + 1;
            if count_digit(n, favorite_digit) >= len.div_ceil(2) {
                total1 += n;
                break;
            }
        }

        let mut cache = vec![u64::MAX; 16 * 16 * favorite_number as usize];
        total2 += dfs(
            0,
            0,
            0,
            favorite_digit,
            favorite_number,
            &(8..=16),
            &mut cache,
        );
    }

    println!("{total1}");
    println!("{total2}");
}
