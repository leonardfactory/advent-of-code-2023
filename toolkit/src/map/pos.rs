use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

pub trait IntoPosValue {
    fn into_coordinate(self) -> i32;
}

impl IntoPosValue for i32 {
    fn into_coordinate(self) -> i32 {
        self
    }
}

impl IntoPosValue for usize {
    fn into_coordinate(self) -> i32 {
        self as i32
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Pos {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<i32> for Pos {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign for Pos {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl MulAssign<i32> for Pos {
    fn mul_assign(&mut self, rhs: i32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Pos {
    pub const ZERO: Pos = Pos { x: 0, y: 0 };

    /// Directions
    pub const NW: Pos = Pos { x: -1, y: -1 };
    pub const N: Pos = Pos { x: 0, y: -1 };
    pub const NE: Pos = Pos { x: 1, y: -1 };
    pub const W: Pos = Pos { x: -1, y: 0 };
    pub const E: Pos = Pos { x: 1, y: 0 };
    pub const SW: Pos = Pos { x: -1, y: 1 };
    pub const S: Pos = Pos { x: 0, y: 1 };
    pub const SE: Pos = Pos { x: 1, y: 1 };

    // Shortcuts
    pub const UP: Pos = Self::N;
    pub const DOWN: Pos = Self::S;
    pub const LEFT: Pos = Self::W;
    pub const RIGHT: Pos = Self::E;

    pub fn new<T: IntoPosValue>(x: T, y: T) -> Self {
        Self {
            x: x.into_coordinate(),
            y: y.into_coordinate(),
        }
    }

    /// Returns only direct neighbors of this position (N, W, E, S) excluding
    /// diagonals
    pub fn neighbors(&self) -> Vec<Pos> {
        vec![
            // North
            *self + Self::N,
            // Center
            *self + Self::W,
            *self + Self::E,
            // Right
            *self + Self::S,
        ]
    }

    /// Returns all neighbors of this position, including diagonals (NE, NW, SE, SW)
    pub fn all_neighbors(&self) -> Vec<Pos> {
        vec![
            // North
            *self + Self::NW,
            *self + Self::N,
            *self + Self::NE,
            // Center
            *self + Self::W,
            *self + Self::E,
            // Right
            *self + Self::SW,
            *self + Self::S,
            *self + Self::SE,
        ]
    }

    pub fn add_y(&self, y: i32) -> Self {
        Self {
            x: self.x,
            y: self.y + y,
        }
    }

    pub fn add_x(&self, x: i32) -> Self {
        Self {
            x: self.x + x,
            y: self.y,
        }
    }

    pub fn opposite(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }

    /**
     * Rotates the position 90 degrees to the left
     */
    pub fn rotate_left(&self) -> Self {
        Self {
            x: self.y,
            y: -self.x,
        }
    }

    /**
     * Rotates the position 90 degrees to the right
     */
    pub fn rotate_right(&self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }

    pub fn as_dir_str(&self) -> String {
        match *self {
            Self::UP => String::from("^"),
            Self::DOWN => String::from("v"),
            Self::LEFT => String::from("<"),
            Self::RIGHT => String::from(">"),
            _ => format!("{:?}", self),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_additions() {
        assert_eq!(Pos::new(1, 1) + Pos::new(1, 1), Pos::new(2, 2));
        assert_eq!(Pos::new(1, 1) + Pos::new(-1, -1), Pos::new(0, 0));
    }

    #[test]
    fn test_rotation() {
        assert_eq!(Pos::UP.rotate_left(), Pos::LEFT);
        assert_eq!(Pos::UP.rotate_right(), Pos::RIGHT);
        assert_eq!(Pos::DOWN.rotate_left(), Pos::RIGHT);
        assert_eq!(Pos::DOWN.rotate_right(), Pos::LEFT);
        assert_eq!(Pos::LEFT.rotate_left(), Pos::DOWN);
        assert_eq!(Pos::LEFT.rotate_right(), Pos::UP);
        assert_eq!(Pos::RIGHT.rotate_left(), Pos::UP);
        assert_eq!(Pos::RIGHT.rotate_right(), Pos::DOWN);
    }
}
