use toolkit::{debug, map::Pos};

use crate::part1::{find_reflection_columns, find_reflection_rows, parse_patterns, Map, Tile};

impl Tile {
    fn swap(&self) -> Tile {
        match self {
            Tile::Rock => Tile::Ash,
            Tile::Ash => Tile::Rock,
        }
    }
}

fn find_alternate_reflection_columns(pattern: &Map) -> (i32, i32) {
    let original_cols = find_reflection_columns(pattern);
    let original_rows = find_reflection_rows(pattern);

    let mut alternate = pattern.clone();
    let mut latest = None;
    for x in 0..pattern.width() {
        for y in 0..pattern.height() {
            if let Some((lx, ly)) = latest {
                alternate
                    .tiles
                    .entry(Pos::new(lx, ly))
                    .and_modify(|t| *t = t.swap());
            }

            latest = Some((x, y));
            alternate
                .tiles
                .entry(Pos::new(x, y))
                .and_modify(|t| *t = t.swap());

            let mut cols = find_reflection_columns(&alternate);
            let mut rows = find_reflection_rows(&alternate);

            if original_cols == cols && original_rows == rows {
                continue;
            }

            if !cols.is_empty() || !rows.is_empty() {
                cols.retain(|c| !original_cols.contains(c));
                rows.retain(|r| !original_rows.contains(r));

                debug!("> Found");
                alternate.print_and_highlight(Pos::new(x, y));
                debug!("Found smudge at ({}, {})", x, y);
                debug!("Cols: {:?}, Rows {:?}", cols, rows);
                return (cols.iter().sum::<i32>(), rows.iter().sum::<i32>());
            }
        }
    }

    panic!("No reflection found");
}

pub fn find_smudged_reflections(input: &str) -> i32 {
    let patterns = parse_patterns(input);
    patterns
        .iter()
        .map(|pattern| {
            // pattern.print();
            let (cols, rows) = find_alternate_reflection_columns(pattern);
            cols + rows * 100
        })
        .sum()
}

#[cfg(test)]
pub mod tests {
    use crate::part2::*;

    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        assert_eq!(find_smudged_reflections(input), 400);
    }
}
