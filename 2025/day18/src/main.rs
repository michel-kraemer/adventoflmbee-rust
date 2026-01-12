use std::fs;

fn matches(pattern: &str, string: &str) -> bool {
    let bp = pattern.as_bytes();
    let bs = string.as_bytes();

    for (&p, &s) in bp.iter().zip(bs.iter()) {
        if p != s && p != b'?' {
            return false;
        }
    }

    true
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let (pattern, string) = input.split_once("\n\n").unwrap();
    let pattern = &pattern[9..].trim();
    let string = &string[8..].trim();

    // part 1
    println!("{}", pattern.lines().filter(|l| matches(l, string)).count());

    // part 2
    let pattern = pattern.replace('\n', "");

    // find all points where the string can overlap itself
    let mut overlap_points = Vec::new();
    for i in 1..string.len() - 1 {
        if string[i..] == string[0..string.len() - i] {
            overlap_points.push(i);
        }
    }

    // bottom-up DP: For each position in the pattern (from the back to the
    // front), determine the maximum number of strings we can place. For this,
    // we need to look up the maximum at each overlap point and all indices that
    // follow the string.
    let mut dp = vec![0; pattern.len()];
    let mut max = vec![0; pattern.len()]; // performance: maintain running maximum
    for i in (0..=pattern.len() - string.len()).rev() {
        if matches(&pattern[i..i + string.len()], string) {
            dp[i] = overlap_points.iter().map(|&j| dp[i + j]).max().unwrap();
            if i + string.len() < dp.len() {
                dp[i] = dp[i].max(max[i + string.len()]);
            }
            dp[i] += 1;
        }
        max[i] = max[i + 1].max(dp[i]);
    }
    println!("{}", dp[0]);
}
