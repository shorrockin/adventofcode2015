use flat::coordinate::Coordinate;

pub fn solve(input: &str, is_part_one: bool) -> usize {
    let mut lights: Vec<usize> = vec![0; 999 * 999];
    input.lines().for_each(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let (command, from, to) = match parts[0] {
            "turn" => match parts[1] {
                "on" => (
                    Command::On(is_part_one),
                    parts[2].split(',').collect::<Vec<_>>(),
                    parts[4].split(',').collect::<Vec<_>>(),
                ),
                "off" => (
                    Command::Off(is_part_one),
                    parts[2].split(',').collect(),
                    parts[4].split(',').collect(),
                ),
                _ => panic!("invalid turn command"),
            },
            "toggle" => (
                Command::Toggle(is_part_one),
                parts[1].split(',').collect(),
                parts[3].split(',').collect(),
            ),
            _ => panic!("invalid prefix"),
        };
        let from = Coordinate(from[0].parse().unwrap(), from[1].parse().unwrap());
        let to = Coordinate(to[0].parse().unwrap(), to[1].parse().unwrap());
        box_iter(from, to).for_each(|c| {
            let index: usize = (c.0 * 999 + c.1).try_into().unwrap();
            lights[index] = command.apply(lights[index]);
        });
    });

    match is_part_one {
        true => lights.iter().sum(),
        false => lights.iter().filter(|&&v| v > 0).sum(),
    }
}

fn box_iter(from: Coordinate, to: Coordinate) -> impl Iterator<Item = Coordinate> {
    (from.0..=to.0).flat_map(move |x| (from.1..=to.1).map(move |y| Coordinate(x, y)))
}

enum Command {
    On(bool),
    Off(bool),
    Toggle(bool),
}
impl Command {
    fn apply(&self, value: usize) -> usize {
        match self {
            Command::On(true) => 1,
            Command::Off(true) => 0,
            Command::Toggle(true) => 1 - value,
            Command::On(false) => value + 1,
            Command::Off(false) => value.saturating_sub(1),
            Command::Toggle(false) => value + 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(543903, solve(INPUT, true));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(14687245, solve(INPUT, false));
    }
}
