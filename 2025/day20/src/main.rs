use std::fs;

struct Point {
    x: i64,
    y: i64,
}

impl From<(i64, i64)> for Point {
    fn from(value: (i64, i64)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Circle {
    x: i64,
    y: i64,
    r: i64,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Rect {
    min_x: i64,
    min_y: i64,
    max_x: i64,
    max_y: i64,
}

impl Rect {
    fn width(&self) -> i64 {
        self.max_x - self.min_x
    }

    fn height(&self) -> i64 {
        self.max_y - self.min_y
    }

    fn area(&self) -> i64 {
        self.width() * self.height()
    }
}

impl From<(i64, i64, i64, i64)> for Rect {
    fn from(value: (i64, i64, i64, i64)) -> Self {
        Self {
            min_x: value.0,
            min_y: value.1,
            max_x: value.2,
            max_y: value.3,
        }
    }
}

trait Contains<T> {
    fn contains(&self, other: &T) -> bool;
}

impl Contains<Point> for Circle {
    fn contains(&self, other: &Point) -> bool {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let squared_dist = dx * dx + dy * dy;
        squared_dist < self.r * self.r
    }
}

trait Overlaps<T> {
    fn overlaps(&self, other: &T) -> bool;
}

impl Overlaps<Circle> for Circle {
    fn overlaps(&self, other: &Circle) -> bool {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let squared_dist = dx * dx + dy * dy;
        squared_dist < (self.r + other.r) * (self.r + other.r)
    }
}

impl Overlaps<Circle> for Rect {
    fn overlaps(&self, other: &Circle) -> bool {
        let x = self.min_x.max(self.max_x.min(other.x));
        let y = self.min_y.max(self.max_y.min(other.y));
        let dx = x - other.x;
        let dy = y - other.y;
        (dx * dx + dy * dy) < other.r * other.r
    }
}

impl Overlaps<Rect> for Circle {
    fn overlaps(&self, other: &Rect) -> bool {
        other.overlaps(self)
    }
}

fn dfs(rect: &Rect, circles: &[Circle], min_overlaps: usize) -> Option<i64> {
    // if the rectangle contains only one point check if this is the point we're
    // looking for
    if rect.width() == 1 && rect.height() == 1 {
        let count = circles
            .iter()
            .filter(|c| c.contains(&Point::from((rect.min_x, rect.min_y))))
            .count();
        if count == min_overlaps {
            return Some(rect.min_x * rect.min_y);
        }
        return None;
    }

    // subdivide rectangle
    let rect1 = Rect::from((
        rect.min_x,
        rect.min_y,
        (rect.min_x + rect.max_x) / 2,
        (rect.min_y + rect.max_y) / 2,
    ));
    let rect2 = Rect::from((
        (rect.min_x + rect.max_x) / 2,
        rect.min_y,
        rect.max_x,
        (rect.min_y + rect.max_y) / 2,
    ));
    let rect3 = Rect::from((
        rect.min_x,
        (rect.min_y + rect.max_y) / 2,
        (rect.min_x + rect.max_x) / 2,
        rect.max_y,
    ));
    let rect4 = Rect::from((
        (rect.min_x + rect.max_x) / 2,
        (rect.min_y + rect.max_y) / 2,
        rect.max_x,
        rect.max_y,
    ));

    // count how many circles overlap with the sub-rectangles
    let mut count1 = 0;
    let mut count2 = 0;
    let mut count3 = 0;
    let mut count4 = 0;

    for c in circles {
        if rect1.area() > 0 && rect1.overlaps(c) {
            count1 += 1;
        }
        if rect2.area() > 0 && rect2.overlaps(c) {
            count2 += 1;
        }
        if rect3.area() > 0 && rect3.overlaps(c) {
            count3 += 1;
        }
        if rect4.area() > 0 && rect4.overlaps(c) {
            count4 += 1;
        }
    }

    // Look in the sub-rectangles for a single point where exactly
    // `min_overlaps` circles overlap. This can only happen if their count is
    // larger than or equal to `min_overlaps`. Otherwise, we don't need to go
    // deeper.
    if count1 >= min_overlaps
        && let Some(r) = dfs(&rect1, circles, min_overlaps)
    {
        return Some(r);
    }
    if count2 >= min_overlaps
        && let Some(r) = dfs(&rect2, circles, min_overlaps)
    {
        return Some(r);
    }
    if count3 >= min_overlaps
        && let Some(r) = dfs(&rect3, circles, min_overlaps)
    {
        return Some(r);
    }
    if count4 >= min_overlaps
        && let Some(r) = dfs(&rect4, circles, min_overlaps)
    {
        return Some(r);
    }

    None
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let mut circles = Vec::new();
    for l in input.lines() {
        let mut parts = l.split_ascii_whitespace();
        let x = parts.next().unwrap();
        let y = parts.next().unwrap();
        let r = parts.next().unwrap();
        let x = x[1..x.len() - 1].parse::<i64>().unwrap();
        let y = y[0..y.len() - 1].parse::<i64>().unwrap();
        let r = r[2..].parse::<i64>().unwrap();
        circles.push(Circle { x, y, r });
    }

    // part 1
    let mut overlaps = vec![0; circles.len()];
    for (i, a) in circles.iter().enumerate() {
        for (j, b) in circles.iter().enumerate().skip(i + 1) {
            if a.overlaps(b) {
                overlaps[i] += 1;
                overlaps[j] += 1;
            }
        }
    }
    let max = overlaps
        .into_iter()
        .enumerate()
        .max_by_key(|(_, o)| *o)
        .unwrap();
    println!("{}", circles[max.0].x * circles[max.0].y + max.1);

    // part 2...

    // find bounding box
    let mut min_x = i64::MAX;
    let mut max_x = i64::MIN;
    let mut min_y = i64::MAX;
    let mut max_y = i64::MIN;
    for c in &circles {
        min_x = min_x.min(c.x - c.r);
        min_y = min_y.min(c.y - c.r);
        max_x = max_x.max(c.x + c.r);
        max_y = max_y.max(c.y + c.r);
    }

    // make bounding box square
    let min = min_x.min(min_y);
    let max = max_x.max(max_y);

    // try to find a single point where `min_overlaps` circles overlap. Try this
    // again and again until we found the point. Since we start with the highest
    // value, the first point we find, will be the one we're looking for.
    let mut min_overlaps = circles.len();
    while min_overlaps > 0 {
        if let Some(r) = dfs(&Rect::from((min, min, max, max)), &circles, min_overlaps) {
            println!("{r}");
            break;
        }
        min_overlaps -= 1;
    }
}
