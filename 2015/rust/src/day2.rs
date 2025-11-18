use std::cmp::min;

struct Present(u32, u32, u32);

impl Present {
    fn paper_needed(&self) -> u32 {
        let a = self.0 * self.1;
        let b = self.1 * self.2;
        let c = self.0 * self.2;

        2 * a + 2 * b + 2 * c + min(min(a, b), c)
    }

    fn ribbon_needed(&self) -> u32 {
        let mut lengths = vec![self.0, self.1, self.2];
        lengths.sort();

        2 * lengths[0] + 2 * lengths[1] + lengths.iter().product::<u32>()
    }
}

pub fn solve(input: &String) {
    let presents: Vec<Present> = input
        .split_terminator("\n")
        .map(|line| {
            let dimensions: Vec<u32> = line.splitn(3, 'x').map(|d| d.parse().unwrap()).collect();
            Present(dimensions[0], dimensions[1], dimensions[2])
        })
        .collect();

    let part1: u32 = presents.iter().map(|x| x.paper_needed()).sum();
    println!("Part 1: {part1}");

    let part2: u32 = presents.iter().map(|x| x.ribbon_needed()).sum();
    println!("Part 2: {part2}");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_paper_needed() {
        assert_eq!(Present(2, 3, 4).paper_needed(), 58);
        assert_eq!(Present(1, 1, 10).paper_needed(), 43);
    }

    #[test]
    fn test_ribbon_needed() {
        assert_eq!(Present(2, 3, 4).ribbon_needed(), 34);
        assert_eq!(Present(1, 1, 10).ribbon_needed(), 14);
    }
}
