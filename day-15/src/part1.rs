pub fn run_hash(input: &str) -> u32 {
    let mut current: u32 = 0;
    input.chars().for_each(|c| {
        current += c as u32;
        current *= 17;
        current %= 256;
    });
    current
}

pub fn hash_init_seq(input: &str) -> u32 {
    input.replace('\n', "").split(',').map(run_hash).sum()
}

#[cfg(test)]
pub mod tests {
    use crate::part1::*;

    #[test]
    fn test_hash() {
        assert_eq!(run_hash("HASH"), 52);
    }

    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        assert_eq!(hash_init_seq(input), 1320);
    }
}
