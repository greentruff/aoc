use regex::Regex;
use std::cmp::max;
use std::cmp::min;

struct Grid {
    lights: [[u32; 1000]; 1000],
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl Grid {
    fn new() -> Grid {
        Grid {
            lights: [[0; 1000]; 1000],
        }
    }
    fn turn_on(&mut self, p: Point) {
        self.lights[p.y][p.x] = 1;
    }
    fn turn_off(&mut self, p: Point) {
        self.lights[p.y][p.x] = 0;
    }
    fn toggle(&mut self, p: Point) {
        if self.lights[p.y][p.x] > 0 {
            self.lights[p.y][p.x] = 0;
        } else {
            self.lights[p.y][p.x] = 1;
        }
    }

    fn brightness_up(&mut self, p: Point, n: u32) {
        self.lights[p.y][p.x] += n;
    }
    fn brightness_down(&mut self, p: Point, n: u32) {
        self.lights[p.y][p.x] = self.lights[p.y][p.x].saturating_sub(n);
    }

    fn count_on(&self) -> u32 {
        self.lights.iter().flatten().sum()
    }
}

struct PointRange {
    start: Point,
    end: Point,
    next: Point,
    done: bool,
}
impl Iterator for PointRange {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let current = self.next;

        if self.end.x > self.next.x {
            self.next.x += 1;
        } else if self.end.y > self.next.y {
            self.next.x = self.start.x;
            self.next.y += 1;
        } else {
            self.done = true;
        }

        Some(current)
    }
}
fn range(from: Point, to: Point) -> PointRange {
    PointRange {
        start: Point {
            x: min(from.x, to.x),
            y: min(from.y, to.y),
        },
        end: Point {
            x: max(from.x, to.x),
            y: max(from.y, to.y),
        },
        next: Point {
            x: min(from.x, to.x),
            y: min(from.y, to.y),
        },
        done: false,
    }
}

enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

struct Operation {
    from: Point,
    to: Point,
    action: Action,
}

fn parse_op(input: &str) -> Operation {
    let re = Regex::new(r"^([a-z ]+) (\d+),(\d+) through (\d+),(\d+)$").unwrap();
    let captures = re.captures(input).unwrap();

    let action = match &captures[1] {
        "turn on" => Action::TurnOn,
        "turn off" => Action::TurnOff,
        "toggle" => Action::Toggle,
        _ => panic!(),
    };

    let from = Point {
        x: captures[2].parse().unwrap(),
        y: captures[3].parse().unwrap(),
    };
    let to = Point {
        x: captures[4].parse().unwrap(),
        y: captures[5].parse().unwrap(),
    };

    Operation { from, to, action }
}

pub fn solve(input: impl AsRef<str>) {
    let mut grid = Grid::new();
    for op in input.as_ref().lines().map(parse_op) {
        for p in range(op.from, op.to) {
            match op.action {
                Action::TurnOn => grid.turn_on(p),
                Action::TurnOff => grid.turn_off(p),
                Action::Toggle => grid.toggle(p),
            }
        }
    }

    let count = grid.count_on();
    println!("Part 1: {count}");

    let mut grid = Grid::new();
    for op in input.as_ref().lines().map(parse_op) {
        for p in range(op.from, op.to) {
            match op.action {
                Action::TurnOn => grid.brightness_up(p, 1),
                Action::TurnOff => grid.brightness_down(p, 1),
                Action::Toggle => grid.brightness_up(p, 2),
            }
        }
    }

    let count = grid.count_on();
    println!("Part 2: {count}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range() {
        assert_eq!(
            range(Point { x: 1, y: 3 }, Point { x: 2, y: 4 }).collect::<Vec<_>>(),
            vec![
                Point { x: 1, y: 3 },
                Point { x: 2, y: 3 },
                Point { x: 1, y: 4 },
                Point { x: 2, y: 4 }
            ]
        )
    }
}
