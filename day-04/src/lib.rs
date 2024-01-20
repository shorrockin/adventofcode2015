pub fn zerod_hash_index(input: &str, part_one: bool) -> usize {
    let mut index = 0;
    loop {
        let digest = md5::compute(format!("{}{}", input, index));

        // a little faster than normal string comparisons
        let found = match part_one {
            true => digest[0] == 0 && digest[1] == 0 && digest[2] <= 15,
            false => digest[0] == 0 && digest[1] == 0 && digest[2] == 0,
        };

        if found {
            return index;
        }

        index += 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_index() {
        assert_eq!(609043, zerod_hash_index("abcdef", true));
        assert_eq!(1048970, zerod_hash_index("pqrstuv", true));
        assert_eq!(117946, zerod_hash_index("ckczppom", true));
        assert_eq!(3938038, zerod_hash_index("ckczppom", false));
    }
}
