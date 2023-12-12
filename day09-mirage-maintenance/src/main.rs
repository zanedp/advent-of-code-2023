fn parse_line(line: &str) -> Vec<i64> {
    line.split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}

fn sequence(history: &[i64]) -> Vec<Vec<i64>> {
    let mut seqs = vec![history.to_vec()];
    let mut all_zero = history.iter().all(|x| *x == 0);
    let mut prev_history = &seqs[0];
    while !all_zero {
        let deltas = prev_history
            .windows(2)
            .map(|w| w[1] - w[0])
            .collect::<Vec<_>>();
        all_zero = deltas.iter().all(|x| *x == 0);
        seqs.push(deltas);
        prev_history = seqs.last().unwrap();
    }
    seqs
}

fn extrapolate(sequence: &[Vec<i64>]) -> i64 {
    let last_elems = sequence.iter().rev().map(|xs| *xs.last().unwrap());
    last_elems.sum()
}

fn part1() {
    // let (input, expected_sum) = (include_str!("sample.txt"), Some(114));
    let (input, expected_sum) = (include_str!("my_input.txt"), Some(1842168671));

    let history_lines = input.lines().map(parse_line);
    let sum = history_lines
        .map(|history| sequence(&history))
        .map(|seq| extrapolate(&seq))
        .sum::<i64>();
    println!("Part 1: {}", sum);
    if let Some(expected_sum) = expected_sum {
        assert_eq!(expected_sum, sum);
    }
}

fn main() {
    part1();
}
