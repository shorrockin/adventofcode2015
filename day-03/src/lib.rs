use flat::coordinate::{Coordinate, Direction};
use std::collections::HashSet;

pub fn part_one(input: &str) -> usize {
    visit(input, 0, 1).len()
}

pub fn part_two(input: &str) -> usize {
    let santa = visit(input, 0, 2);
    let robot = visit(input, 1, 2);
    santa.union(&robot).count()
}

pub fn visit(input: &str, offset: usize, step: usize) -> HashSet<Coordinate> {
    let mut current = Coordinate(0, 0);

    input
        .chars()
        .enumerate()
        .filter(|(idx, _)| (idx + offset) % step == 0)
        .fold(HashSet::from([current]), |mut visited, (_, c)| {
            current = match c {
                '^' => current + Direction::North,
                '>' => current + Direction::East,
                'v' => current + Direction::South,
                '<' => current + Direction::West,
                _ => panic!("unexpected character"),
            };
            visited.insert(current);
            visited
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(2, part_one(">"));
        assert_eq!(4, part_one("^>v<"));
        assert_eq!(2, part_one("^v^v^v^v^v"));
        assert_eq!(2565, part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(3, part_two("^v"));
        assert_eq!(3, part_two("^>v<"));
        assert_eq!(11, part_two("^v^v^v^v^v"));
        assert_eq!(2639, part_two(INPUT));
    }
}
