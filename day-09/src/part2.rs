use crate::part1::{compute_deltas, is_zero_delta, parse, Sequence};

pub fn postdict(seq: &Sequence) -> i32 {
    println!("Postdict {:?}", seq);
    let mut deltas: Vec<Sequence> = vec![seq.clone()];
    loop {
        let delta = compute_deltas(deltas.last().unwrap().clone());
        if is_zero_delta(&delta) {
            break;
        }
        deltas.push(delta);
    }

    let mut postdiction = 0;
    for seq in deltas.iter().rev() {
        postdiction = seq.first().unwrap() - postdiction;
        // println!("{:?} -> {}", seq, postdiction);
    }
    postdiction
}

pub fn sum_postdictions(input: &str) -> i32 {
    let seqs = parse(input);
    seqs.iter().map(postdict).sum()
}

#[cfg(test)]
pub mod tests {
    use crate::part2::*;

    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        assert_eq!(sum_postdictions(input), 2);
    }
}
