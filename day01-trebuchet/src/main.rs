fn first_and_last_digit(line: &str) -> (u32, u32) {
    (first_digit(line), last_digit(line))
}

fn first_digit(line: &str) -> u32 {
    let mappings = vec![
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    let mut buf = String::new();
    for c in line.chars() {
        if c.is_ascii_digit() {
            return c.to_digit(10).unwrap();
        }
        buf.push(c);
        for (word, digit) in &mappings {
            if buf.contains(word) {
                return *digit;
            }
        }
    }
    panic!("no digit found in line: {}", line);
}

fn last_digit(line: &str) -> u32 {
    let mappings = vec![
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
    .iter()
    .map(|(word, digit)| (word.chars().rev().collect::<String>(), *digit))
    .collect::<Vec<_>>();

    let mut buf = String::new();
    for c in line.chars().rev() {
        if c.is_ascii_digit() {
            return c.to_digit(10).unwrap();
        }
        buf.push(c);
        for (word, digit) in &mappings {
            if buf.contains(word) {
                return *digit;
            }
        }
    }
    panic!("no digit found in line: {}", line);
}

#[test]
fn test_first_digit() {
    assert_eq!(first_digit("zero"), 0);
    assert_eq!(first_digit("z0"), 0);
    assert_eq!(first_digit("eightwo"), 8);
    assert_eq!(first_digit("oneight"), 1);
    assert_eq!(first_digit("on2eight"), 2);
    assert_eq!(first_digit("twone"), 2);
}

#[test]
fn test_last_digit() {
    assert_eq!(last_digit("zero"), 0);
    assert_eq!(last_digit("z0"), 0);
    assert_eq!(last_digit("eightwo"), 2);
    assert_eq!(last_digit("oneight"), 8);
    assert_eq!(last_digit("on2eight"), 8);
    assert_eq!(last_digit("twone"), 1);
}

fn part1() {
    // let (input, expected_sum) = (include_str!("sample1.txt"), Some(142));
    let (input, expected_sum) = (include_str!("my_input.txt"), Some(56108));
    let sum: u32 = input
        .lines()
        .map(|line| {
            let digits = line
                .chars()
                .filter(char::is_ascii_digit)
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>();
            let (left, right) = (*digits.first().unwrap(), *digits.last().unwrap());
            (left, right)
        })
        .map(|(left, right)| left * 10 + right)
        .sum();
    println!("part 1 sum: {}", sum);
    if expected_sum.is_some() {
        assert_eq!(sum, expected_sum.unwrap());
    }
}

fn part2() {
    // let (input, expected_sum) = (include_str!("sample2.txt"), Some(281));
    let (input, expected_sum) = (include_str!("my_input.txt"), Some(55652));

    let sum: u32 = input
        .lines()
        .map(first_and_last_digit)
        .map(|(left, right)| left * 10 + right)
        .sum();
    println!("part 2 sum: {}", sum);
    if expected_sum.is_some() {
        assert_eq!(sum, expected_sum.unwrap());
    }
}

fn main() {
    part1();
    part2();
}
