use std::{collections::HashMap, mem::swap};

fn parse(input: &str) -> Vec<(usize, usize)> {
    let mut cosmos = vec![];
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                cosmos.push((i + 1, j + 1));
            }
        }
    }
    cosmos
}

fn expand(cosmos: &Vec<(usize, usize)>, factor: usize) -> Vec<(usize, usize)> {
    let mut fat_cosmos = Vec::with_capacity(cosmos.len());
    let mut previous_row = 1;
    let mut expansion = 0;
    for (r, c) in cosmos.iter() {
        let distance_from_previous_row = r - previous_row;
        expansion = if distance_from_previous_row > 0 {
            expansion + ((distance_from_previous_row - 1) * (factor - 1))
        } else {
            expansion
        };
        let new_coord = (r + expansion, *c);
        fat_cosmos.push(new_coord);
        previous_row = *r;
    }

    // swap and sort so that we can do the same thing for columns
    fat_cosmos.iter_mut().for_each(|(r, c)| {
        swap(r, c);
    });
    fat_cosmos.sort_unstable();

    // basically a copy-paste of the above loop
    let mut previous_row = 1;
    let mut expansion = 0;
    for (r, c) in fat_cosmos.iter_mut() {
        let distance_from_previous_row = *r - previous_row;
        expansion = if distance_from_previous_row > 0 {
            expansion + ((distance_from_previous_row - 1) * (factor - 1))
        } else {
            expansion
        };
        previous_row = *r;
        let new_coord = (*r + expansion, *c);
        (*r, *c) = new_coord;
    }

    // swap back
    fat_cosmos.iter_mut().for_each(|(r, c)| {
        swap(r, c);
    });
    fat_cosmos.sort_unstable();

    fat_cosmos
}

#[test]
fn test_expand() {
    let sample1_cosmos = parse(include_str!("sample1.txt"));
    assert_eq!(
        sample1_cosmos,
        expand(&sample1_cosmos, 1),
        "factor 1 (no-op)"
    );

    let expected = vec![
        (1, 5),
        (2, 10),
        (3, 1),
        (6, 9),
        (7, 2),
        (8, 13),
        (11, 10),
        (12, 1),
        (12, 6),
    ];
    let result = expand(&sample1_cosmos, 2);
    assert_eq!(result, expected, "factor 2");

    let input = indoc::indoc! {
        "
        ##.#
        ....
        .#.#
        "
    };
    let cosmos = parse(input);
    assert_eq!(cosmos, vec![(1, 1), (1, 2), (1, 4), (3, 2), (3, 4)]);

    let expected2 = indoc::indoc! {
        "
        ##..#
        .....
        .....
        .#..#
        "
    };

    // let expected1: Vec<(usize, usize)> = vec![(1, 1), (1, 3), (1, 6), (4, 3), (4, 6)];
    assert_eq!(expand(&cosmos, 2), parse(expected2), "factor 2");

    // let expected2 = vec![(1, 4), (1, 8), (5, 2), (5, 4)];
    let expected3 = indoc::indoc! {
        "
        ##...#
        ......
        ......
        ......
        .#...#
        "
    };
    assert_eq!(expand(&cosmos, 3), parse(expected3), "factor 3");
}

fn distance(a: (usize, usize), b: (usize, usize)) -> usize {
    let (x1, y1) = (a.0 as isize, a.1 as isize);
    let (x2, y2) = (b.0 as isize, b.1 as isize);

    ((x1 - x2).abs() + (y1 - y2).abs()).unsigned_abs()
}

#[test]
fn test_distance() {
    assert_eq!(distance((7, 2), (12, 6)), 9); // 5 and 9
    assert_eq!(distance((1, 5), (11, 10)), 15); // 1 and 7
    assert_eq!(distance((3, 1), (8, 13)), 17); // 3 and 6
    assert_eq!(distance((12, 1), (12, 6)), 5); // 8 and 9
}

fn part1(input: &str) -> usize {
    let cosmos = parse(input);
    let fat_cosmos = expand(&cosmos, 2);

    let mut distances: HashMap<(usize, usize), HashMap<(usize, usize), usize>> = HashMap::new();
    for src in fat_cosmos.iter() {
        for dst in fat_cosmos.iter() {
            if dst == src || distances.contains_key(dst) {
                continue;
            }

            let distance = distance(*src, *dst);
            distances
                .entry(*src)
                .or_default()
                .entry(*dst)
                .or_insert(distance);
        }
    }

    let sum_of_shortests = distances
        .values()
        .map(|dsts| dsts.values().sum::<usize>())
        .sum();

    sum_of_shortests
}

fn part2(input: &str, expansion_factor: usize) -> usize {
    let cosmos = parse(input);
    let fat_cosmos = expand(&cosmos, expansion_factor);

    let mut distances: HashMap<(usize, usize), HashMap<(usize, usize), usize>> = HashMap::new();
    for src in fat_cosmos.iter() {
        for dst in fat_cosmos.iter() {
            if dst == src || distances.contains_key(dst) {
                continue;
            }

            let distance = distance(*src, *dst);
            distances
                .entry(*src)
                .or_default()
                .entry(*dst)
                .or_insert(distance);
        }
    }

    let sum_of_shortests = distances
        .values()
        .map(|dsts| dsts.values().sum::<usize>())
        .sum();

    sum_of_shortests
}

fn main() {
    // let (input, expected) = (include_str!("sample1.txt"), Some(374_usize));
    let (input, expected) = (include_str!("my_input.txt"), Some(9647174));
    let p1_result = part1(input);
    println!("part1: {}", p1_result);
    if let Some(expected) = expected {
        assert_eq!(p1_result, expected);
    }

    // let (input, factor, expected) = (include_str!("sample1.txt"), 10, Some(1030));
    // let (input, factor, expected) = (include_str!("sample1.txt"), 100, Some(8410));
    let (input, factor, expected) = (include_str!("my_input.txt"), 1_000_000, Some(377318892554));
    let p2_result = part2(input, factor);
    println!("part2: {}", p2_result);
    if let Some(expected) = expected {
        assert_eq!(p2_result, expected);
    }
}
