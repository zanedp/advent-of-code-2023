use std::iter::once;

fn read_schematic(input: &str) -> Vec<Vec<char>> {
    let mut retval = Vec::new();
    let mut lines = input.lines();

    // read the first line so we know how long the lines are
    // and can make a margin of dots where the first and last lines
    // are just dots.
    let first_line = lines.next().unwrap();
    let first_and_last_margin = vec!['.'; first_line.len() + 2];

    // add the margin at the top of the schematic
    retval.push(first_and_last_margin.clone());

    // add margin of one char to the left and right of first line
    let padded_first_line = once('.')
        .chain(first_line.chars())
        .chain(once('.'))
        .collect();
    retval.push(padded_first_line);

    // add margin to all other lines
    for line in lines {
        let padded_line = once('.').chain(line.chars()).chain(once('.')).collect();
        retval.push(padded_line);
    }

    // add the margin at the bottom of the schematic
    retval.push(first_and_last_margin);
    retval
}

#[allow(dead_code)]
fn print_schematic(schematic: &Vec<Vec<char>>) {
    for row in schematic {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    None,
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Direction {
    fn offset(&self) -> (i32, i32) {
        match self {
            Direction::None => (0, 0),
            Direction::N => (1, 0),
            Direction::NE => (1, 1),
            Direction::E => (0, 1),
            Direction::SE => (-1, 1),
            Direction::S => (-1, 0),
            Direction::SW => (-1, -1),
            Direction::W => (0, -1),
            Direction::NW => (1, -1),
        }
    }

    fn next(&self) -> Direction {
        match self {
            Direction::None => Direction::N,
            Direction::N => Direction::NE,
            Direction::NE => Direction::E,
            Direction::E => Direction::SE,
            Direction::SE => Direction::S,
            Direction::S => Direction::SW,
            Direction::SW => Direction::W,
            Direction::W => Direction::NW,
            Direction::NW => Direction::None,
        }
    }
}

fn is_adjacent_to_symbol(schematic: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    fn is_symbol(c: char) -> bool {
        c != '.' && !c.is_ascii_digit()
    }

    let mut direction = Direction::N;

    while direction != Direction::None {
        let (row_offset, col_offset) = direction.offset();
        let neighbor_row = row as i32 + row_offset;
        let neighbor_col = col as i32 + col_offset;

        if is_symbol(schematic[neighbor_row as usize][neighbor_col as usize]) {
            return true;
        }

        direction = direction.next();
    }
    false
}

fn find_part_numbers(schematic: &Vec<Vec<char>>) -> Vec<u32> {
    let mut retval = Vec::new();

    let mut buf = String::new();
    let mut is_part_number = false;

    for row in 0..schematic.len() {
        for col in 0..schematic[row].len() {
            let cur_char = schematic[row][col];
            if cur_char.is_ascii_digit() {
                buf.push(schematic[row][col]);
                is_part_number |= is_adjacent_to_symbol(schematic, row, col);
            } else {
                // we are at the end of a number.
                // if it's a part number, we'll add it to the list
                // and then go looking for the next number
                if is_part_number {
                    retval.push(buf.parse::<u32>().unwrap());
                }
                buf.clear();
                is_part_number = false;
            }
        }
    }
    retval
}

fn part1() {
    // let (input, expected_sum) = (include_str!("sample1.txt"), Some(4361));
    let (input, expected_sum) = (include_str!("my_input.txt"), None);
    let schematic = &read_schematic(input);
    let sum: u32 = find_part_numbers(schematic).iter().sum();
    println!("part 1 sum: {}", sum);
    if expected_sum.is_some() {
        assert_eq!(sum, expected_sum.unwrap());
    }
}

fn main() {
    part1();
}
