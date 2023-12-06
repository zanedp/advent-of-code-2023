use std::{collections::HashMap, iter::once};

#[derive(Debug)]
struct Mapping {
    dest_start: u64,
    src_start: u64,
    src_end: u64,
}

impl Mapping {
    fn new(dest_start: u64, src_start: u64, len: u64) -> Self {
        Self {
            dest_start,
            src_start,
            src_end: src_start + len,
        }
    }

    fn map(&self, src: u64) -> Option<u64> {
        if src < self.src_start || src > self.src_end {
            None
        } else {
            let delta = src - self.src_start;
            Some(self.dest_start + delta)
        }
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    mappings: HashMap<MappingOp, Vec<Mapping>>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum MappingOp {
    None,
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

impl MappingOp {
    fn next(&self) -> Self {
        use MappingOp::*;
        match self {
            None => SeedToSoil,
            SeedToSoil => SoilToFertilizer,
            SoilToFertilizer => FertilizerToWater,
            FertilizerToWater => WaterToLight,
            WaterToLight => LightToTemperature,
            LightToTemperature => TemperatureToHumidity,
            TemperatureToHumidity => HumidityToLocation,
            HumidityToLocation => None,
        }
    }
}

fn parse_input(text: &str) -> Almanac {
    use MappingOp::*;

    // add an empty line to the end so we can detect the end of the last mapping
    let mut lines = text.lines().chain(once(""));

    let seeds_line = lines.next().unwrap();
    let (seeds_heading, seeds_list) = seeds_line
        .split_once(':')
        .expect("should have found `seeds: <numbers>`");
    assert_eq!(
        "seeds", seeds_heading,
        "should have found `seeds: <numbers>`"
    );
    let seeds: Vec<u64> = seeds_list
        .split_whitespace()
        .map(|seed| seed.trim().parse().expect("should have found a number"))
        .collect();

    let mut mappings = HashMap::new();
    let mut cur_mapping_name = MappingOp::None;
    let mut cur_mappings: Vec<Mapping> = Vec::new();
    for line in lines {
        if line == "seed-to-soil map:" {
            cur_mapping_name = SeedToSoil;
        } else if line == "soil-to-fertilizer map:" {
            cur_mapping_name = SoilToFertilizer;
        } else if line == "fertilizer-to-water map:" {
            cur_mapping_name = FertilizerToWater;
        } else if line == "water-to-light map:" {
            cur_mapping_name = WaterToLight;
        } else if line == "light-to-temperature map:" {
            cur_mapping_name = LightToTemperature;
        } else if line == "temperature-to-humidity map:" {
            cur_mapping_name = TemperatureToHumidity;
        } else if line == "humidity-to-location map:" {
            cur_mapping_name = HumidityToLocation;
        } else if line.is_empty() {
            if cur_mapping_name != None {
                mappings.insert(cur_mapping_name, cur_mappings);
                cur_mapping_name = None;
                cur_mappings = Vec::new();
            }
        } else {
            // if it's not one of the headers, it must be the 3-number mapping line
            let mut parts = line.split_whitespace();
            let dest_start: u64 = parts
                .next()
                .and_then(|x| x.trim().parse().ok())
                .expect("should have found a destination number");
            let src_start: u64 = parts
                .next()
                .and_then(|x| x.trim().parse().ok())
                .expect("should have found a src number");
            let len: u64 = parts
                .next()
                .and_then(|x| x.trim().parse().ok())
                .expect("should have found a length");
            let mapping = Mapping::new(dest_start, src_start, len);
            cur_mappings.push(mapping);
        }
    }
    Almanac { seeds, mappings }
}

fn part1() {
    // let (text, expected_min) = (include_str!("sample.txt"), Some(35));
    let (text, expected_min) = (include_str!("my_input.txt"), None);
    let almanac = parse_input(text);
    let mut destinations = Vec::new();

    for seed_num in almanac.seeds {
        let mut cur_mapping_operation = MappingOp::SeedToSoil;
        let mut cur_src = seed_num;
        while cur_mapping_operation != MappingOp::None {
            let mappings = almanac.mappings.get(&cur_mapping_operation).unwrap();
            let dest = mappings
                .iter()
                .filter_map(|mapping| mapping.map(cur_src))
                .next();
            let next_src = dest.unwrap_or(cur_src);

            cur_mapping_operation = cur_mapping_operation.next();
            cur_src = next_src;
        }
        println!("seed {} ends up at {}", seed_num, cur_src);
        destinations.push(cur_src);
    }

    let min = *destinations
        .iter()
        .min()
        .expect("should have found a location");

    println!("lowest location is {}", min);
    if let Some(expected_min) = expected_min {
        assert_eq!(min, expected_min);
    }
}

fn main() {
    part1();
}
