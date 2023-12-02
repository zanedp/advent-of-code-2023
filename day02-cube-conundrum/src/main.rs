#[derive(Debug, Default, PartialEq, Eq)]
struct GameRecord {
    id: u32,
    subsets: Vec<Subset>,
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Subset {
    blue: u32,
    red: u32,
    green: u32,
}

impl GameRecord {
    fn is_possible(&self, test_condition: &Subset) -> bool {
        self.subsets.iter().all(|x| x.is_possible(test_condition))
    }
}

impl Subset {
    fn is_possible(&self, test_condition: &Subset) -> bool {
        self.blue <= test_condition.blue
            && self.red <= test_condition.red
            && self.green <= test_condition.green
    }
}

fn parse_line(line: &str) -> GameRecord {
    fn parse_subset(subset: &str) -> Subset {
        let mut retval = Subset::default();
        let color_info = subset
            .split(',')
            .map(|x| x.trim())
            .map(|x| x.split_once(' ').expect("should have a count and a color"));

        for (count, color) in color_info {
            let count = count.parse::<u32>().unwrap();
            match color {
                "blue" => retval.blue = count,
                "red" => retval.red = count,
                "green" => retval.green = count,
                _ => panic!("Invalid color"),
            }
        }
        retval
    }

    let (game_id, subsets_list) = line.split_once(':').unwrap();
    let game_id = game_id.split_once(' ').unwrap().1.parse::<u32>().unwrap();
    let subsets = subsets_list.split(';').map(parse_subset).collect();

    GameRecord {
        id: game_id,
        subsets,
    }
}

#[test]
fn test_parse_line() {
    let input = "Game 1: 3 blue, 4 red; 2 red, 1 green; 1 blue, 2 green";
    let expected = GameRecord {
        id: 1,
        subsets: vec![
            Subset {
                blue: 3,
                red: 4,
                green: 0,
            },
            Subset {
                blue: 0,
                red: 2,
                green: 1,
            },
            Subset {
                blue: 1,
                red: 0,
                green: 2,
            },
        ],
    };
    assert_eq!(parse_line(input), expected);
}

fn part1() {
    // let (input, expected_sum) = (include_str!("sample1.txt"), Some(8));
    let (input, expected_sum) = (include_str!("my_input.txt"), Some(1867));
    let test_condition = Subset {
        red: 12,
        green: 13,
        blue: 14,
    };
    let input = input.lines().map(parse_line).collect::<Vec<_>>();
    let possible_games = input
        .iter()
        .filter(|x| x.is_possible(&test_condition))
        .collect::<Vec<_>>();
    let sum: u32 = possible_games.iter().map(|x| x.id).sum();
    println!("part 1 sum: {sum}");
    assert_eq!(Some(sum), expected_sum);
}

fn main() {
    part1();
}
