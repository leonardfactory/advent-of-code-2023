use std::ops::RangeInclusive;

use super::Pos;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Bounds {
    pub min: Pos,
    pub max: Pos,
}

impl Bounds {
    pub fn new(min: Pos, max: Pos) -> Self {
        Self { min, max }
    }

    pub fn empty() -> Self {
        Self {
            min: Pos::ZERO,
            max: Pos::ZERO,
        }
    }

    pub fn width(&self) -> i32 {
        self.max.x - self.min.x + 1
    }

    pub fn height(&self) -> i32 {
        self.max.y - self.min.y + 1
    }

    pub fn x_range(&self) -> RangeInclusive<i32> {
        self.min.x..=self.max.x
    }

    pub fn y_range(&self) -> RangeInclusive<i32> {
        self.min.y..=self.max.y
    }

    pub fn contains(&self, pos: Pos) -> bool {
        pos.x >= self.min.x && pos.x <= self.max.x && pos.y >= self.min.y && pos.y <= self.max.y
    }

    /**
     * Expand the bounds to include the given position
     */
    pub fn insert_pos(&mut self, pos: Pos) {
        self.min.x = self.min.x.min(pos.x);
        self.min.y = self.min.y.min(pos.y);
        self.max.x = self.max.x.max(pos.x);
        self.max.y = self.max.y.max(pos.y);
    }
}
