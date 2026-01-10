use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let mut even1 = 0;
    let mut odd1 = 0;
    let mut total1 = 0;

    let mut even2 = Vec::new();
    let mut odd2 = Vec::new();

    for l in input.lines() {
        if l.starts_with("plant") {
            let height = l[6..].parse::<i64>().unwrap();
            total1 += height;
            if height % 2 == 0 {
                even1 += 1;
                even2.push(height);
            } else {
                odd1 += 1;
                odd2.push(height);
            }
        } else if &l[6..] == "even" {
            total1 += even1;
            odd1 += even1;
            even1 = 0;

            let mut i = 0;
            while i < even2.len() {
                even2[i] /= 2;
                if even2[i] % 2 != 0 {
                    odd2.push(even2.swap_remove(i));
                } else {
                    i += 1;
                }
            }
        } else if &l[6..] == "odd" {
            total1 += odd1;
            even1 += odd1;
            odd1 = 0;

            let mut i = 0;
            while i < odd2.len() {
                odd2[i] /= 2;
                if odd2[i] % 2 == 0 {
                    even2.push(odd2.swap_remove(i));
                } else {
                    i += 1;
                }
            }
        } else {
            total1 += even1 + odd1;
            (even1, odd1) = (odd1, even1);

            let mut new_even2 = Vec::new();
            let mut new_odd2 = Vec::new();

            let mut i = 0;
            while i < even2.len() {
                even2[i] /= 2;
                if even2[i] % 2 != 0 {
                    new_odd2.push(even2.swap_remove(i));
                } else {
                    i += 1;
                }
            }
            i = 0;
            while i < odd2.len() {
                odd2[i] /= 2;
                if odd2[i] % 2 == 0 {
                    new_even2.push(odd2.swap_remove(i));
                } else {
                    i += 1;
                }
            }

            even2.extend(new_even2);
            odd2.extend(new_odd2);
        }
    }

    println!("{total1}");
    println!("{}", even2.iter().sum::<i64>() + odd2.iter().sum::<i64>());
}
