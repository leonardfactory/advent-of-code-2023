use crate::part1::run_hash;

pub type Lens = u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Label {
    pub label: u32,
    pub hash: usize,
}

impl Label {
    pub fn new(label: &str) -> Self {
        let label_hash = label.chars().fold(0, |acc, c| acc * 256 + c as u32);
        Self {
            label: label_hash,
            hash: run_hash(label) as usize,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Op {
    Remove(Label),
    Set(Label, Lens),
}

#[derive(Debug, Clone)]
pub struct Box {
    pub lenses: Vec<(Label, Lens)>,
}

pub fn parse_ops(input: &str) -> Vec<Op> {
    input
        .split(',')
        .map(|raw_op| {
            if let Some((label, _)) = raw_op.split_once('-') {
                Op::Remove(Label::new(label))
            } else if let Some((label, lens)) = raw_op.split_once('=') {
                Op::Set(Label::new(label), lens.parse().unwrap())
            } else {
                panic!("Invalid op: {}", raw_op);
            }
        })
        .collect()
}

pub fn run_op(boxes: &mut [Box], op: Op) {
    match op {
        Op::Remove(label) => {
            let target = boxes.get_mut(label.hash).unwrap();
            target.lenses.retain(|(l, _)| *l != label);
        }
        Op::Set(label, lens) => {
            let target = boxes.get_mut(label.hash).unwrap();
            match target.lenses.iter().position(|(l, _)| *l == label) {
                Some(idx) => target.lenses[idx].1 = lens,
                None => target.lenses.push((label, lens)),
            }
        }
    }
}

pub fn focusing_power(input: &str) -> u32 {
    let mut boxes = vec![Box { lenses: vec![] }; 256];
    parse_ops(input)
        .iter()
        .for_each(|op| run_op(&mut boxes, *op));

    boxes
        .iter()
        .enumerate()
        .flat_map(|(i, b)| {
            b.lenses
                .iter()
                .enumerate()
                .map(move |(j, (label, lens))| (i as u32 + 1) * (j as u32 + 1) * *lens as u32)
        })
        .sum::<u32>()
}

#[cfg(test)]
pub mod tests {
    use crate::part2::*;

    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        assert_eq!(focusing_power(input), 145);
    }
}
