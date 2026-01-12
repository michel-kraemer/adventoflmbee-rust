use std::fs;

#[derive(Clone, Copy)]
enum Event {
    Start(u64, u64),
    End(u64),
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let mut events = Vec::new();
    for l in input.lines() {
        let mut parts = l.split_ascii_whitespace();
        let start = parts.nth(6).unwrap().parse::<u64>().unwrap();
        let end = parts.last().unwrap().parse::<u64>().unwrap();
        events.push(Event::Start(start, end));
        events.push(Event::End(end));
    }
    events.sort_unstable_by_key(|&e| match e {
        Event::Start(time, _) => time,
        Event::End(time) => time,
    });

    // Solve both parts simultaneously. For part 1, we start a lesson but switch
    // immediately to another one if it finishes earlier as the one we're
    // currently in. This does not change the number of lesson we've visited but
    // gives us more time. For part 2, we simply compute the maximum number of
    // lessons that run in parallel at any given time.

    let mut total1 = 1;
    let mut total2 = 0;

    let mut curr_end = if let Event::Start(_, end) = events[0] {
        end
    } else {
        unreachable!()
    };
    let mut curr_sum = 1;

    for &e in events.iter().skip(1) {
        match e {
            Event::Start(start, end) => {
                if start < curr_end {
                    curr_end = curr_end.min(end);
                } else {
                    total1 += 1;
                    curr_end = end;
                }
                curr_sum += 1;
                total2 = total2.max(curr_sum);
            }
            Event::End(_) => {
                curr_sum -= 1;
            }
        }
    }

    println!("{total1}");
    println!("{total2}");
}
