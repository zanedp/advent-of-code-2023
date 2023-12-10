use std::collections::HashMap;

fn parse_input(input: &str) -> (Vec<char>, HashMap<String, (String, String)>) {
    let mut graph = HashMap::new();
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().chars().collect();
    lines.next(); // blank line
    for line in lines {
        let mut parts = line.split(" = ");
        let node_name = parts.next().unwrap();
        let children = parts.next().unwrap().replace(['(', ')'], "");
        let (left, right) = children.split_once(',').unwrap();
        graph.insert(
            node_name.to_string(),
            (left.trim().to_string(), right.trim().to_string()),
        );
    }
    (instructions, graph)
}

fn part1() {
    // let (input, expected_steps) = (include_str!("sample1-1.txt"), Some(2));
    // let (input, expected_steps) = (include_str!("sample1-2.txt"), Some(6));
    let (input, expected_steps) = (include_str!("my_input.txt"), Some(19637));
    let (instructions, graph) = parse_input(input);

    let mut num_steps = 0_u32;
    let mut cur_node = &String::from("AAA");
    let mut instructions = instructions.iter().cycle();
    while cur_node != "ZZZ" {
        num_steps += 1;
        let (left, right) = graph.get(cur_node).unwrap();
        match instructions.next().unwrap() {
            'L' => cur_node = left,
            'R' => cur_node = right,
            _ => panic!("Invalid instruction"),
        }
    }

    println!("Part 1: {}", num_steps);
    if let Some(expected_steps) = expected_steps {
        assert_eq!(expected_steps, num_steps);
    }
}

fn part2() {
    // let (input, expected_steps) = (include_str!("sample2.txt"), Some(6));
    // let (input, expected_steps) = (include_str!("sample1-2.txt"), Some(6));
    let (input, expected_steps) = (include_str!("my_input.txt"), None::<u64>);
    let (instructions, graph) = parse_input(input);

    let mut cur_nodes = graph
        .iter()
        .filter(|(node, _)| node.ends_with('A'))
        .map(|(node, _)| node)
        .collect::<Vec<_>>();
    // dbg!(&cur_nodes);

    let mut num_steps = 0_u64;
    let mut instructions = instructions.iter().cycle();
    while !cur_nodes.iter().all(|node| node.ends_with('Z')) {
        num_steps += 1;
        let cur_instruction = instructions.next().unwrap();
        if num_steps % 1000000 == 0 {
            println!("{} - {}: {:?}", cur_instruction, num_steps, cur_nodes);
        }
        for cur_node in cur_nodes.iter_mut() {
            let (left, right) = graph.get(*cur_node).unwrap();
            *cur_node = match cur_instruction {
                'L' => left,
                'R' => right,
                _ => panic!("Invalid instruction"),
            };
        }

        // println!("{}: {:?}", num_steps, cur_nodes);
    }

    println!("Part 2: {}", num_steps);
    if let Some(expected_steps) = expected_steps {
        assert_eq!(expected_steps, num_steps);
    }
}

fn main() {
    part1();
    part2();
}
