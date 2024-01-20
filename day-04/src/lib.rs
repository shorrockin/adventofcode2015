use rayon::prelude::*;

const CHUNK_SIZE: usize = 500_000;

// rayon is a parallel iterator library for Rust, however it doesn't work
// on unbound ranges, so we chunk it up. this is in contrast to using a max
// range which feels like it should work, however, due to how tasks are split
// up, the performance benefits are not great since it does not optimize for
// finding values in the lower end of the max range.
pub fn find_index(input: &str, part_one: bool) -> usize {
    let mut chunk = 0;

    loop {
        let from = chunk * CHUNK_SIZE;
        let to = (chunk + 1) * CHUNK_SIZE;

        match find_index_between(input, part_one, from, to) {
            Some(index) => return index,
            None => chunk += 1,
        }
    }
}

fn find_index_between(input: &str, part_one: bool, from: usize, to: usize) -> Option<usize> {
    (from..to).into_par_iter().find_first(|index| {
        let digest = md5::compute(format!("{}{}", input, index));

        // a little faster than normal string comparisons (15 == 0f)
        match part_one {
            true => digest[0] == 0 && digest[1] == 0 && digest[2] <= 15,
            false => digest[0] == 0 && digest[1] == 0 && digest[2] == 0,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(609043, find_index("abcdef", true));
        assert_eq!(1048970, find_index("pqrstuv", true));
        assert_eq!(117946, find_index("ckczppom", true));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(3938038, find_index("ckczppom", false));
    }
}
