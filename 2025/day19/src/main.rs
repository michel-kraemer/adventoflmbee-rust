use std::fs;

fn dp(
    grid: &[u64],
    width: usize,
    height: usize,
    start: (usize, usize),
    end: (usize, usize),
    dx: isize,
    dy: isize,
) -> Vec<u64> {
    let mut best = vec![0; width * height];

    let mut sy = start.1 as isize;
    while sy != end.1 as isize + dy {
        let y = sy as usize;

        let mut sx = start.0 as isize;
        while sx != end.0 as isize + dx {
            let x = sx as usize;

            let a = if sx != start.0 as isize {
                best[y * width + (sx - dx) as usize]
            } else {
                0
            };
            let b = if sy != start.1 as isize {
                best[(sy - dy) as usize * width + x]
            } else {
                0
            };
            best[y * width + x] = grid[y * width + x] + a.max(b);

            sx += dx;
        }

        sy += dy;
    }

    best
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let mut total1 = 1;
    let mut total2 = 1;
    for b in input.split("\n\n") {
        let lines = b.lines().collect::<Vec<_>>();
        let width = lines[0].len();
        let height = lines.len();
        let grid = lines
            .into_iter()
            .flat_map(|l| l.bytes().map(|b| (b - b'0') as u64))
            .collect::<Vec<_>>();

        // perform BFS from each corner to its diagonally opposite one top left
        // to bottom right:
        let best_top_left = dp(&grid, width, height, (0, 0), (width - 1, height - 1), 1, 1);

        // bottom right to top left:
        let best_bottom_right = dp(
            &grid,
            width,
            height,
            (width - 1, height - 1),
            (0, 0),
            -1,
            -1,
        );

        // bottom left to top right
        let best_bottom_left = dp(&grid, width, height, (0, height - 1), (width - 1, 0), 1, -1);

        // top right to bottom left
        let best_top_right = dp(&grid, width, height, (width - 1, 0), (0, height - 1), -1, 1);

        // part 1
        total1 *= best_top_left[height * width - 1];

        // part 2 - for every possible intersection point between the paths,
        // compute the maximum from each corner to the intersection point
        let mut max = 0;
        for y in 1..height - 1 {
            for x in 1..width - 1 {
                let center = grid[y * width + x];

                //       v
                //      R_1
                // ->R_2-|-R_2->
                //      R_1
                //       v
                {
                    let max1_1 = best_top_left[(y - 1) * width + x];
                    let max1_2 = best_bottom_right[(y + 1) * width + x];
                    let max2_1 = best_bottom_left[y * width + x - 1];
                    let max2_2 = best_top_right[y * width + x + 1];
                    max = max.max(center * 2 + max1_1 + max1_2 + max2_1 + max2_2);
                }

                //       ^
                //      R_2
                // ->R_1-|-R_1->
                //      R_2
                //       ^
                {
                    let max1_1 = best_top_left[y * width + x - 1];
                    let max1_2 = best_bottom_right[y * width + x + 1];
                    let max2_1 = best_bottom_left[(y + 1) * width + x];
                    let max2_2 = best_top_right[(y - 1) * width + x];
                    max = max.max(center * 2 + max1_1 + max1_2 + max2_1 + max2_2);
                }
            }
        }
        total2 *= max;
    }

    println!("{total1}");
    println!("{total2}");
}
