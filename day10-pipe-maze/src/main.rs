use std::ops::Add;
use std::ops::AddAssign;

fn add_margin(s: &str) -> String {
    let mut result = String::new();
    let mut lines = s.lines();
    let first_line = lines.next().unwrap();
    let len = first_line.len();
    let blank_line = &".".repeat(len + 2);
    result.push_str(blank_line);
    result.push('\n');

    let first_line_padded = format!(".{}.", first_line);
    result.push_str(&first_line_padded);
    result.push('\n');
    for line in lines {
        let padded_line = format!(".{}.", line);
        result.push_str(&padded_line);
        result.push('\n');
    }
    result.push_str(blank_line);
    result.push('\n');
    result
}

#[derive(Clone)]
struct PipeMaze {
    maze: Vec<Vec<PipeSection>>,
    start: (usize, usize),
}

impl PipeMaze {
    /// Returns a new maze with the given location marked with an X
    #[allow(dead_code)]
    fn with_location(&self, (row, col): (usize, usize)) -> Self {
        let mut new_maze = self.maze.clone();
        new_maze[row][col] = PipeSection::Marker;
        Self {
            maze: new_maze,
            start: self.start,
        }
    }
}

impl std::str::FromStr for PipeMaze {
    type Err = ();

    fn from_str(s: &str) -> Result<PipeMaze, Self::Err> {
        let maze = Self::read_maze(s);
        let start = Self::find_start(&maze);
        Ok(PipeMaze { maze, start })
    }
}

impl std::fmt::Display for PipeMaze {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in &self.maze {
            for col in row {
                if f.alternate() {
                    write!(f, "{:#}", col)?;
                } else {
                    write!(f, "{}", col)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl PipeMaze {
    pub fn pipe_section_at(&self, (row, col): (usize, usize)) -> PipeSection {
        self.maze[row][col]
    }

    fn start_exit_directions(&self) -> (Direction, Direction) {
        use Direction::*;
        let mut next_dir = Some(North);
        let mut first = None;
        let mut second = None;
        while let Some(cur_dir) = next_dir {
            let neighbor = self.start + cur_dir;
            let neighbor_pipe = self.pipe_section_at(neighbor);
            if neighbor_pipe.has_entrance_from(cur_dir.flip()) {
                if first.is_none() {
                    first = Some(cur_dir);
                } else if second.is_none() {
                    second = Some(cur_dir);
                } else {
                    panic!("Too many exit directions");
                }
            }
            next_dir = cur_dir.next();
        }
        (first.unwrap(), second.unwrap())
    }

    fn read_maze(maze_str: &str) -> Vec<Vec<PipeSection>> {
        maze_str
            .lines()
            .map(|line| line.chars().map(PipeSection::from_char).collect())
            .collect()
    }

    fn find_start(maze: &[Vec<PipeSection>]) -> (usize, usize) {
        for (row, line) in maze.iter().enumerate() {
            for (col, section) in line.iter().enumerate() {
                if *section == PipeSection::Start {
                    return (row, col);
                }
            }
        }
        panic!("No start found");
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PipeSection {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
    Marker,
}

impl PipeSection {
    fn from_char(c: char) -> PipeSection {
        match c {
            '|' => PipeSection::NS,
            '-' => PipeSection::EW,
            'L' => PipeSection::NE,
            'J' => PipeSection::NW,
            '7' => PipeSection::SW,
            'F' => PipeSection::SE,
            '.' => PipeSection::Ground,
            'S' => PipeSection::Start,
            'X' => PipeSection::Marker,
            _ => panic!("Unknown pipe section: {}", c),
        }
    }

    fn exit_direction(&self, entry_direction: Direction) -> Direction {
        use Direction::*;
        use PipeSection::*;
        match (self, entry_direction) {
            (NS, North) => South,
            (NS, South) => North,
            (EW, East) => West,
            (EW, West) => East,
            (NE, North) => East,
            (NE, East) => North,
            (NW, North) => West,
            (NW, West) => North,
            (SW, South) => West,
            (SW, West) => South,
            (SE, South) => East,
            (SE, East) => South,
            (_, _) => panic!(
                "Unexpected pipe section and entry direction: {:?} {:?}",
                self, entry_direction
            ),
        }
    }

    fn has_entrance_from(&self, entry_direction: Direction) -> bool {
        use Direction::*;
        use PipeSection::*;
        match (self, entry_direction) {
            (NS, North) | (NS, South) => true,
            (NE, North) | (NE, East) => true,
            (NW, North) | (NW, West) => true,
            (SW, South) | (SW, West) => true,
            (SE, South) | (SE, East) => true,
            (EW, East) | (EW, West) => true,
            (_, _) => false,
        }
    }

    #[allow(dead_code)]
    fn endpoints(&self) -> impl Iterator<Item = Direction> {
        use Direction::*;
        use PipeSection::*;
        match self {
            NS => vec![North, South],
            EW => vec![East, West],
            NE => vec![North, East],
            NW => vec![North, West],
            SW => vec![South, West],
            SE => vec![South, East],
            Ground => vec![],
            Start => vec![],
            Marker => panic!("Marker has no endpoints"),
        }
        .into_iter()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn next(&self) -> Option<Direction> {
        use Direction::*;
        match self {
            North => Some(East),
            East => Some(South),
            South => Some(West),
            West => None,
        }
    }

    pub fn flip(&self) -> Direction {
        use Direction::*;
        match self {
            North => South,
            South => North,
            East => West,
            West => East,
        }
    }
}

impl AddAssign<Direction> for (usize, usize) {
    fn add_assign(&mut self, rhs: Direction) {
        *self = *self + rhs;
    }
}

impl Add<Direction> for (usize, usize) {
    type Output = (usize, usize);

    fn add(self, rhs: Direction) -> Self::Output {
        use Direction::*;
        let (row, col) = self;
        match rhs {
            North => (row - 1, col),
            South => (row + 1, col),
            East => (row, col + 1),
            West => (row, col - 1),
        }
    }
}

impl From<char> for PipeSection {
    fn from(c: char) -> PipeSection {
        PipeSection::from_char(c)
    }
}

impl std::fmt::Display for PipeSection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use PipeSection::*;
        let c = if f.alternate() {
            match self {
                SE => '┌',
                NS => '│',
                NE => '└',
                EW => '─',
                NW => '┘',
                SW => '┐',
                Ground => '.',
                Start => 'S',
                Marker => 'X',
            }
        } else {
            match self {
                NS => '|',
                EW => '-',
                NE => 'L',
                NW => 'J',
                SW => '7',
                SE => 'F',
                Ground => '.',
                Start => 'S',
                Marker => 'X',
            }
        };
        write!(f, "{}", c)
    }
}

fn part1() {
    // let (input, expected_steps) = (include_str!("sample1a.txt"), Some(4));
    // let (input, expected_steps) = (include_str!("sample1b.txt"), Some(8));
    let (input, expected_steps) = (include_str!("my_input.txt"), Some(6931));
    let maze: PipeMaze = add_margin(input).parse().unwrap();
    println!("{:#}", maze);
    // travel directions
    let (mut dir0, mut dir1) = maze.start_exit_directions();
    let mut pos0 = maze.start + dir0;
    let mut pos1 = maze.start + dir1;
    let mut num_steps = 1;
    while pos0 != pos1 {
        // println!("Num steps: {}", num_steps);
        // println!("{:#}", maze.with_location(pos0).with_location(pos1));
        // println!();

        let pipe0 = maze.pipe_section_at(pos0);
        let pipe1 = maze.pipe_section_at(pos1);
        let next_dir0 = pipe0.exit_direction(dir0.flip());
        let next_dir1 = pipe1.exit_direction(dir1.flip());

        pos0 += next_dir0;
        pos1 += next_dir1;
        dir0 = next_dir0;
        dir1 = next_dir1;
        num_steps += 1;
    }
    println!("part 1 num steps: {}", num_steps);
    if let Some(expected_steps) = expected_steps {
        assert_eq!(num_steps, expected_steps);
    }
}

/// Calculates the area of a polygon using the trapezoid method of the [Shoelace formula](https://en.wikipedia.org/wiki/Shoelace_formula)
fn polygon_area_trapezoid(path: &[(usize, usize)]) -> f64 {
    let signed_area = path
        .windows(2)
        .map(|pair| {
            let (x0, y0) = pair[0];
            let (x1, y1) = pair[1];
            let x0 = x0 as f64;
            let x1 = x1 as f64;
            let y0 = y0 as f64;
            let y1 = y1 as f64;
            (y0 + y1) * (x0 - x1)
        })
        .sum::<f64>()
        / 2.0;
    signed_area.abs()
}

fn part2() {
    // let (input, expected_contained_tiles) = (include_str!("sample2a.txt"), Some(4));
    // let (input, expected_contained_tiles) = (include_str!("sample2b.txt"), Some(8_usize));
    // let (input, expected_contained_tiles) = (include_str!("sample2c.txt"), Some(10));
    let (input, expected_contained_tiles) = (include_str!("my_input.txt"), Some(357));
    let maze: PipeMaze = add_margin(input).parse().unwrap();
    println!("{:#}", maze);
    // travel directions
    let (mut dir, _) = maze.start_exit_directions();

    // follow the maze, counting the length of the path
    // for fun, we'll X-out the path as we go.
    let mut marked_maze = maze.clone();
    let mut route = Vec::new();
    route.push(maze.start);
    marked_maze.maze[maze.start.0][maze.start.1] = PipeSection::Marker;
    let mut pos = maze.start + dir;
    while pos != maze.start {
        route.push(pos);
        let pipe = maze.pipe_section_at(pos);
        let next_dir = pipe.exit_direction(dir.flip());
        marked_maze.maze[pos.0][pos.1] = PipeSection::Marker;
        pos += next_dir;
        dir = next_dir;
    }
    route.push(maze.start); // to complete the loop, need to return to the start

    // Use Pick's theorem to count the number of tiles inside the polygon.
    // https://en.wikipedia.org/wiki/Pick%27s_theorem
    // First we need to total area of the polygon formed by the route of the
    // pipes (A):
    let total_area = polygon_area_trapezoid(&route);
    // The number of segments in the pipe route is the number of boundary points (b)
    let b = route.len();
    // i = A - (b/2) + 1
    let internal_points = (total_area as usize) - (b / 2) + 1;

    println!("part 2 internal points: {}", internal_points);
    if let Some(expected_tiles) = expected_contained_tiles {
        assert_eq!(internal_points, expected_tiles);
    }
}

fn main() {
    part1();
    part2();
}
