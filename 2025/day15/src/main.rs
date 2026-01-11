use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let mut giveaways = 0;
    let mut total1 = 0;
    let mut toys1: Vec<u64> = Vec::new();

    let mut total2 = 0;
    let mut toys2: Vec<(u64, u64)> = Vec::new();
    let mut len2 = 0;

    for l in input.lines() {
        if l.starts_with("receive") {
            let (_, b) = l.split_once(' ').unwrap();
            let b = b.parse::<u64>().unwrap();

            // part 1
            let i = toys1.partition_point(|i| *i < b);
            toys1.insert(i, b);

            // part 2
            let i = toys2.partition_point(|i| i.0 < b);
            if i < toys2.len() && toys2[i].0 == b {
                toys2[i].1 += b;
            } else {
                toys2.insert(i, (b, b));
            }
            len2 += b;
        } else {
            giveaways += 1;

            // part 1
            let median = toys1.len() / 2;
            total1 += giveaways * toys1.remove(median);

            // part 2
            let median2 = len2 / 2;

            let mut i = 0;
            let mut pos = 0;
            while i < toys2.len() && pos + toys2[i].1 <= median2 {
                pos += toys2[i].1;
                i += 1;
            }

            let v = toys2[i].0;
            toys2[i].1 -= 1;
            total2 += giveaways * v;
            len2 -= 1;

            if toys2[i].1 == 0 {
                toys2.remove(i);
            }
        }
    }

    println!("{total1}");
    println!("{total2}");
}
