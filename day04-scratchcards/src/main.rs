fn parse_card(line: &str) -> (Vec<u32>, Vec<u32>) {
    let numbers = line.split_once(':').unwrap().1;
    let (winners, haves) = numbers.split_once('|').unwrap();
    let winners = winners
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let haves = haves
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    (winners, haves)
}

fn part1() {
    // let (input, expected_sum) = (include_str!("sample.txt"), Some(13));
    let (input, expected_sum) = (include_str!("my_input.txt"), Some(23235));
    let cards = input.lines().map(parse_card).collect::<Vec<_>>();
    let winners = cards
        .iter()
        .map(|(winners, haves)| {
            haves
                .iter()
                .filter(|x| winners.contains(x))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    // number of winning numbers on each card
    let winning_counts = winners.iter().map(|x| x.len()).collect::<Vec<_>>();
    fn calc_score(winning_count: usize) -> usize {
        if winning_count == 0 {
            0
        } else {
            1usize << (winning_count - 1)
        }
    }
    let scores = winning_counts.iter().map(|x| calc_score(*x));
    let sum: usize = scores.sum();

    println!("part 1 sum: {}", sum);
    if expected_sum.is_some() {
        assert_eq!(expected_sum.unwrap(), sum);
    }
}

fn part2() {
    // let (input, expected_sum) = (include_str!("sample.txt"), Some(30));
    let (input, expected_sum) = (include_str!("my_input.txt"), Some(5920640));

    // list of winners and haves for each card
    let cards = input.lines().map(parse_card).collect::<Vec<_>>();
    let winners = cards
        .iter()
        .map(|(winners, haves)| {
            haves
                .iter()
                .filter(|x| winners.contains(x))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // number of winning numbers on each card
    let winning_counts = winners.iter().map(|x| x.len()).collect::<Vec<_>>();

    let mut card_counts = vec![1; cards.len()];
    for i in 0..winning_counts.len() {
        let win_count = winning_counts[i];
        let card_count = card_counts[i];
        // we get an extra card for each of the next <win_count> cards
        card_counts
            .iter_mut()
            .skip(i + 1)
            .take(win_count)
            .for_each(|x| *x += card_count);
    }
    let sum: usize = card_counts.iter().sum();

    println!("part 2 sum: {}", sum);
    if expected_sum.is_some() {
        assert_eq!(expected_sum.unwrap(), sum);
    }
}

fn main() {
    part1();
    part2();
}
