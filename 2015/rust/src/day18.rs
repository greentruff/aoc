use crate::aoc::Solution;

pub fn solve(input: &str) -> Solution {
    let mut grid = Grid::parse(input);
    for _ in 0..100 {
        grid.advance();
    }
    let part1 = grid.count();

    let mut grid = Grid::parse(input);
    for _ in 0..100 {
        grid.advance_with_corners();
    }
    let part2 = grid.count();

    Solution::new(18, &part1, &part2)
}

struct Grid {
    data: Box<[[bool; 100]; 100]>,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let mut data = [[false; 100]; 100];

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                data[y][x] = c == '#';
            }
        }
        Self { data: Box::from(data) }
    }

    fn count(&self) -> usize {
        self.data
            .map(|line| line.iter().filter(|x| **x).count())
            .iter()
            .sum()
    }

    fn advance(&mut self) {
        let mut next = [[false; 100]; 100];
        for x in 0..100_i32 {
            for y in 0..100_i32 {
                #[rustfmt::skip]
                let neighbour_points = [
                    (x-1, y-1), (x, y-1), (x+1, y-1),
                    (x-1, y),             (x+1, y),
                    (x-1, y+1), (x, y+1), (x+1, y+1),
                ];
                let neighbours = neighbour_points
                    .iter()
                    .filter(|(px, py)| (0..100).contains(px) && (0..100).contains(py))
                    .filter(|(px, py)| self.data[*py as usize][*px as usize])
                    .count();

                let xx = x as usize;
                let yy = y as usize;
                if self.data[yy][xx] {
                    next[yy][xx] = neighbours == 2 || neighbours == 3;
                } else {
                    next[yy][xx] = neighbours == 3;
                }
            }
        }

        self.data = Box::from(next);
    }

    fn advance_with_corners(&mut self) {
        for (x, y) in [(0, 0), (0, 99), (99, 0), (99, 99)] {
            self.data[y][x] = true;
        }
        self.advance();
        for (x, y) in [(0, 0), (0, 99), (99, 0), (99, 99)] {
            self.data[y][x] = true;
        }
    }
}
