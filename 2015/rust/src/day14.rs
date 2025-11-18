use crate::aoc::Solution;
use crate::day14::State::{Flying, Resting};

const RACE_TIME: u32 = 2503;

pub fn solve(input: &str) -> Solution {
    let reindeer = input.lines().map(parse_line).collect::<Vec<_>>();

    let part1 = most_distance(&reindeer, RACE_TIME);
    let part2 = most_score(&reindeer, RACE_TIME);

    Solution::new(14, &part1, &part2)
}

fn most_distance(reindeer: &[Reindeer], race_time: u32) -> u32 {
    reindeer
        .iter()
        .map(|r| distance_for_time(r, race_time))
        .max()
        .unwrap()
}

fn distance_for_time(reindeer: &Reindeer, time: u32) -> u32 {
    let cycle_time = reindeer.fly_time + reindeer.rest_time;
    let full_cycles = time / cycle_time;

    let remaining_time = time % cycle_time;
    let remaining_distance = u32::min(remaining_time, reindeer.fly_time) * reindeer.speed;

    full_cycles * (reindeer.fly_time * reindeer.speed) + remaining_distance
}

fn most_score(reindeer: &[Reindeer], race_time: u32) -> u32 {
    let mut racers = reindeer
        .iter()
        .map(|r| Racer {
            reindeer: r,
            distance: 0,

            state: Flying,
            since: 0,

            score: 0,
        })
        .collect::<Vec<_>>();

    for _ in 0..race_time {
        let mut max_distance = 0;

        for racer in &mut racers {
            racer.step();
            if racer.distance > max_distance {
                max_distance = racer.distance;
            }
        }

        for racer in &mut racers {
            if racer.distance == max_distance {
                racer.score += 1;
            }
        }
    }

    racers.iter().map(|r| r.score).max().unwrap()
}

struct Reindeer {
    speed: u32,
    fly_time: u32,
    rest_time: u32,
}

enum State {
    Flying,
    Resting,
}
struct Racer<'a> {
    reindeer: &'a Reindeer,
    score: u32,
    state: State,
    since: u32,
    distance: u32,
}

impl Racer<'_> {
    fn step(&mut self) {
        self.since += 1;
        match self.state {
            Flying => {
                self.distance += self.reindeer.speed;
                if self.since >= self.reindeer.fly_time {
                    self.since = 0;
                    self.state = Resting;
                }
            }
            Resting => {
                if self.since >= self.reindeer.rest_time {
                    self.since = 0;
                    self.state = Flying;
                }
            }
        }
    }
}

fn parse_line(line: &str) -> Reindeer {
    let parts = line.split_whitespace().collect::<Vec<_>>();
    Reindeer {
        speed: parts[3].parse().unwrap(),
        fly_time: parts[6].parse().unwrap(),
        rest_time: parts[13].parse().unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use crate::day14::{most_distance, most_score, parse_line};

    const SAMPLE: &str = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";

    #[test]
    fn test_distance() {
        let reindeer = SAMPLE.lines().map(parse_line).collect::<Vec<_>>();
        assert_eq!(most_distance(&reindeer, 1000), 1120);
    }

    #[test]
    fn test_score() {
        let reindeer = SAMPLE.lines().map(parse_line).collect::<Vec<_>>();
        assert_eq!(most_score(&reindeer, 1000), 689);
    }
}
