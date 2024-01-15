pub fn part_one(input: &str) -> i32 {
    floors(input).last().unwrap()
}

pub fn part_two(input: &str) -> usize {
    floors(input).position(|floor| floor == -1).unwrap() + 1
}

fn floors(input: &str) -> impl Iterator<Item = i32> + '_ {
    input.chars().scan(0, |acc, c| {
        *acc += match c {
            '(' => 1,
            ')' => -1,
            _ => panic!("unexpected character"),
        };
        Some(*acc)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_parts() {
        assert_eq!(74, part_one(INPUT));
        assert_eq!(1795, part_two(INPUT));
    }
}
