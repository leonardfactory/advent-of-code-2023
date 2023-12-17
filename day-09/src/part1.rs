pub type Sequence = Vec<i32>;

pub fn parse(input: &str) -> Vec<Sequence> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}

pub fn compute_deltas(seq: Sequence) -> Sequence {
    let mut deltas = Vec::with_capacity(seq.len() - 1);
    for i in 0..seq.len() - 1 {
        deltas.push(seq[i + 1] - seq[i]);
    }
    deltas
}

pub fn is_zero_delta(seq: &Sequence) -> bool {
    seq.iter().all(|&x| x == 0)
}

pub fn predict(seq: &Sequence) -> i32 {
    println!("Predict {:?}", seq);
    let mut deltas: Vec<Sequence> = vec![seq.clone()];
    loop {
        let delta = compute_deltas(deltas.last().unwrap().clone());
        if is_zero_delta(&delta) {
            break;
        }
        deltas.push(delta);
    }

    let mut prediction = 0;
    for seq in deltas.iter().rev() {
        prediction += seq.last().unwrap();
        // println!("{:?} -> {}", seq, prediction);
    }
    prediction
}

pub fn sum_predictions(input: &str) -> i32 {
    let seqs = parse(input);
    seqs.iter().map(predict).sum()
}

#[cfg(test)]
pub mod tests {
    use crate::part1::*;

    #[test]
    fn test_predict() {
        let input = "0   3   6   9  12  15";
        let seq = parse(input);
        assert_eq!(predict(seq.last().unwrap()), 18);
    }

    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        assert_eq!(sum_predictions(input), 114);
    }
}
