fn quadratic_formula(a: f64, b: f64, c: f64) -> (f64, f64) {
    let discriminant = b * b - 4.0 * a * c;
    let x1 = (-b + discriminant.sqrt()) / (2.0 * a);
    let x2 = (-b - discriminant.sqrt()) / (2.0 * a);
    (x2, x1)
}

fn calc_button_limits(time_limit: u64, distance_record: u64) -> (u64, u64) {
    let time_limit = time_limit as f64;
    let distance_record = distance_record as f64;
    let (min, max) = quadratic_formula(1.0, -time_limit, distance_record);
    (
        (min + 0.000001).ceil() as u64,
        (max - 0.000001).floor() as u64,
    )
}

fn parse_input(input: &str) -> (Vec<u64>, Vec<u64>) {
    fn parse_line(line: &str) -> Vec<u64> {
        line.split(':')
            .nth(1)
            .expect("should have found <header>: <list of integers>")
            .split_whitespace()
            .flat_map(|x| x.parse().ok())
            .collect()
    }
    let mut lines = input.lines();
    let times = parse_line(lines.next().expect("should have found Times:"));
    let distances = parse_line(lines.next().expect("should have found Distances:"));
    (times, distances)
}

fn part1() {
    // let (input, expected_prod) = (include_str!("sample.txt"), Some(288u64));
    let (input, expected_prod) = (include_str!("my_input.txt"), None::<u64>);
    let (times, distances) = parse_input(input);
    let times_and_distances = Iterator::zip(times.iter(), distances.iter());

    let product = times_and_distances
        .map(|(&t, &d)| calc_button_limits(t, d))
        .map(|(min, max)| max - min + 1) // number of solutions
        .product();

    println!("Part 1: {}", product);
    if let Some(expected_prod) = expected_prod {
        assert_eq!(expected_prod, product);
    }
}

fn main() {
    part1();
}
