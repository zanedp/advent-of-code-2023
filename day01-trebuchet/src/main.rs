fn main() {
    // let input = include_str!("sample.txt");
    let input = include_str!("my_input.txt");
    let sum: u32 = input
        .lines()
        .map(|line| {
            let digits = line
                .chars()
                .filter(char::is_ascii_digit)
                .map(|c| c.to_digit(10).expect("that is not a digit!"))
                .collect::<Vec<_>>();
            let (left, right) = (
                digits.first().unwrap().to_owned(),
                digits.last().unwrap().to_owned(),
            );
            (left, right)
        })
        .inspect(|&x| println!("{:?}", x))
        .map(|(left, right)| left * 10 + right)
        .inspect(|&x| println!("{:?}", x))
        .sum();
    println!("sum: {}", sum);
}
