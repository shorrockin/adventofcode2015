pub fn part_one(input: &str) -> usize {
    parse(input)
        .map(|present| present.area() + present.smallest_side())
        .sum()
}

pub fn part_two(input: &str) -> usize {
    parse(input)
        .map(|present| present.smallest_perimeter() + present.volume())
        .sum()
}

fn parse(input: &str) -> impl Iterator<Item = Present> + '_ {
    input
        .lines()
        .map(|line| {
            line.split('x')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<usize>>()
        })
        .map(|lwh| Present::new(lwh[0], lwh[1], lwh[2]))
}

struct Present {
    length: usize,
    width: usize,
    height: usize,
}
impl Present {
    fn new(length: usize, width: usize, height: usize) -> Self {
        Present {
            length,
            width,
            height,
        }
    }

    fn area(&self) -> usize {
        2 * self.length * self.width + 2 * self.width * self.height + 2 * self.height * self.length
    }

    fn smallest_side(&self) -> usize {
        let mut sides = [self.length, self.width, self.height];
        sides.sort();
        sides[0] * sides[1]
    }

    fn volume(&self) -> usize {
        self.length * self.width * self.height
    }

    fn smallest_perimeter(&self) -> usize {
        let mut sides = [self.length, self.width, self.height];
        sides.sort();
        2 * sides[0] + 2 * sides[1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(58, part_one("2x3x4"));
        assert_eq!(43, part_one("1x1x10"));
        assert_eq!(1598415, part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(34, part_two("2x3x4"));
        assert_eq!(14, part_two("1x1x10"));
        assert_eq!(3812909, part_two(INPUT));
    }
}
