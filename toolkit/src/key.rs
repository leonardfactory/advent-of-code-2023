use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Key(u32);

impl FromStr for Key {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.chars()
                .enumerate()
                .map(|(i, c)| (c as u32) << (i * 8))
                .sum(),
        ))
    }
}

impl PartialEq<&str> for Key {
    fn eq(&self, other: &&str) -> bool {
        self == &Key::from_str(other).unwrap()
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut n = self.0;
        while n > 0 {
            write!(f, "{}", (n) as u8 as char)?;
            n >>= 8;
        }
        Ok(())
    }
}

#[cfg(test)]
pub mod tests {
    use crate::key::Key;

    #[test]
    fn test_from() {
        assert_eq!("AAA", "AAA".parse::<Key>().unwrap().to_string());
    }
}
