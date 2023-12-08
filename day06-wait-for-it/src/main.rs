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
    ((min + 0.05).ceil() as u64, (max - 0.05).floor() as u64)
}

fn part1() {
    use std::iter::zip;

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

    // let (input, expected_prod) = (include_str!("sample.txt"), Some(288_u64));
    let (input, expected_prod) = (include_str!("my_input.txt"), Some(140220_u64));
    let (times, distances) = parse_input(input);
    let times_and_distances = zip(times.iter(), distances.iter());

    let product = times_and_distances
        .map(|(&t, &d)| calc_button_limits(t, d))
        .map(|(min, max)| max - min + 1) // number of solutions
        .product();

    println!("Part 1: {}", product);
    if let Some(expected_prod) = expected_prod {
        assert_eq!(expected_prod, product);
    }
}

fn part2() {
    fn parse_input(input: &str) -> (u64, u64) {
        fn parse_line(line: &str) -> u64 {
            let (_header, rest) = line
                .split_once(':')
                .expect("should have found <header>: <list of integers>");
            rest.replace(' ', "")
                .parse()
                .expect("should have found an integer")
        }
        let mut lines = input.lines();
        let time = parse_line(lines.next().expect("should have found Times:"));
        let distance = parse_line(lines.next().expect("should have found Distances:"));
        (time, distance)
    }
    // let (input, expected_ways) = (include_str!("sample.txt"), Some(71503_u64));
    let (input, expected_ways) = (include_str!("my_input.txt"), Some(39570185_u64));
    let (time, distance) = parse_input(input);
    let (min, max) = calc_button_limits(time, distance);
    let num_ways_to_win = max - min + 1;

    println!("Part 2: {}", num_ways_to_win);
    if let Some(expected_ways) = expected_ways {
        assert_eq!(expected_ways, num_ways_to_win);
    }
}

fn main() {
    part1();
    part2();
}
