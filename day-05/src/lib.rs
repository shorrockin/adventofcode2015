pub fn part_one(input: &str) -> usize {
    input.lines().filter(|l| part_one_nice(l)).count()
}

fn part_one_nice(input: &str) -> bool {
    let mut vowel_count = 0;
    let mut has_double = false;
    let mut has_bad = false;

    for (i, c) in input.chars().enumerate() {
        if i > 0 {
            let prev = input.chars().nth(i - 1).unwrap();
            if prev == c {
                has_double = true;
            }
            has_bad = match (prev, c) {
                ('a', 'b') => true,
                ('c', 'd') => true,
                ('p', 'q') => true,
                ('x', 'y') => true,
                _ => has_bad,
            };
        }
        if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
            vowel_count += 1;
        }
    }
    vowel_count >= 3 && has_double && !has_bad
}

pub fn part_two(input: &str) -> usize {
    input.lines().filter(|l| part_two_nice(l)).count()
}

fn part_two_nice(input: &str) -> bool {
    let mut has_pair = false;
    let mut has_repeat = false;

    for (i, c) in input.chars().enumerate() {
        if i > 1 {
            let second_prev = input.chars().nth(i - 1).unwrap();
            let first_prev = input.chars().nth(i - 2).unwrap();

            if first_prev == c {
                has_repeat = true;
            }
            if has_pair || input[i..].contains(&format!("{}{}", first_prev, second_prev)) {
                has_pair = true;
            }
        }
    }
    has_pair && has_repeat
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("ugknbfddgicrmopn"), 1);
        assert_eq!(part_one("aaa"), 1);
        assert_eq!(part_one("jchzalrnumimnmhp"), 0);
        assert_eq!(part_one("haegwjzuvuyypxyu"), 0);
        assert_eq!(part_one("dvszwmarrgswjxmb"), 0);
        assert_eq!(part_one(INPUT), 255);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("qjhvhtzxzqqjkmpb"), 1);
        assert_eq!(part_two("aaaba"), 0);
        assert_eq!(part_two("xxyxx"), 1);
        assert_eq!(part_two("uurcxstgmygtbstg"), 0);
        assert_eq!(part_two("ieodomkazucvgmuy"), 0);
        assert_eq!(part_two("aabcdefegaa"), 1);
        assert_eq!(part_two(INPUT), 55);
    }
}
