pub fn part_one(input: &str) -> i32 {
    *floors(input).last().unwrap()
}

pub fn part_two(input: &str) -> usize {
    floors(input).iter().position(|floor| *floor == -1).unwrap() + 1
}

fn floors(input: &str) -> Vec<i32> {
    input
        .chars()
        .scan(0, |acc, c| {
            *acc += match c {
                '(' => 1,
                ')' => -1,
                _ => panic!("unexpected character"),
            };
            Some(*acc)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(74, part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(1795, part_two(INPUT));
    }
}
