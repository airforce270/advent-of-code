use std::collections::HashSet;

use crate::custom_error::AocError;

#[derive(Debug)]
enum CurrentBlock {
    Start,
    Seeds,
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemp,
    TempToHumidity,
    HumidityToLocation,
}

struct Mapping {
    src_start: u64,
    src_end: u64,
    dest_start: u64,
}

struct Mappings {
    m: Vec<Mapping>,
}

impl Mappings {
    fn new() -> Mappings {
        Mappings { m: vec![] }
    }
    fn add(&mut self, m: Mapping) {
        self.m.push(m);
    }
    fn get(&self, num: u64) -> u64 {
        for mapping in &self.m {
            if num < mapping.src_start || num > mapping.src_end {
                continue;
            }
            return mapping.dest_start + (num - mapping.src_start);
        }
        // No mapping present
        num
    }
}

pub fn process(input: &str) -> std::result::Result<String, AocError> {
    let mut seeds: HashSet<u64> = HashSet::new();
    let mut seed_to_soil = Mappings::new();
    let mut soil_to_fertilizer = Mappings::new();
    let mut fertilizer_to_water = Mappings::new();
    let mut water_to_light = Mappings::new();
    let mut light_to_temp = Mappings::new();
    let mut temp_to_humidity = Mappings::new();
    let mut humidity_to_location = Mappings::new();

    let mut current_block = CurrentBlock::Start;

    for line in input.lines() {
        match line {
            ln if ln.starts_with("seeds:") => {
                current_block = CurrentBlock::Seeds;
                ln.split(":")
                    .skip(1)
                    .next()
                    .unwrap()
                    .trim()
                    .split(" ")
                    .map(|num| num.trim())
                    .filter(|num| !num.is_empty())
                    .map(|num| num.parse().unwrap())
                    .for_each(|num| {
                        seeds.insert(num);
                    })
            }
            ln if ln.starts_with("seed-to-soil") => {
                current_block = CurrentBlock::SeedToSoil;
            }
            ln if ln.starts_with("soil-to-fertilizer") => {
                current_block = CurrentBlock::SoilToFertilizer;
            }
            ln if ln.starts_with("fertilizer-to-water") => {
                current_block = CurrentBlock::FertilizerToWater;
            }
            ln if ln.starts_with("water-to-light") => {
                current_block = CurrentBlock::WaterToLight;
            }
            ln if ln.starts_with("light-to-temperature") => {
                current_block = CurrentBlock::LightToTemp;
            }
            ln if ln.starts_with("temperature-to-humidity") => {
                current_block = CurrentBlock::TempToHumidity;
            }
            ln if ln.starts_with("humidity-to-location") => {
                current_block = CurrentBlock::HumidityToLocation;
            }
            ln if ln.is_empty() => {
                continue;
            }
            ln => match current_block {
                CurrentBlock::SeedToSoil => parse_line_to_mappings(ln, &mut seed_to_soil),
                CurrentBlock::SoilToFertilizer => {
                    parse_line_to_mappings(ln, &mut soil_to_fertilizer)
                }
                CurrentBlock::FertilizerToWater => {
                    parse_line_to_mappings(ln, &mut fertilizer_to_water)
                }
                CurrentBlock::WaterToLight => parse_line_to_mappings(ln, &mut water_to_light),
                CurrentBlock::LightToTemp => parse_line_to_mappings(ln, &mut light_to_temp),
                CurrentBlock::TempToHumidity => parse_line_to_mappings(ln, &mut temp_to_humidity),
                CurrentBlock::HumidityToLocation => {
                    parse_line_to_mappings(ln, &mut humidity_to_location)
                }
                CurrentBlock::Start | CurrentBlock::Seeds => {
                    return Err(AocError {
                        details: format!("unexpected line {} in block {:?}", ln, current_block),
                    });
                }
            },
        }
    }

    let location_nums: HashSet<u64> = seeds
        .iter()
        .copied()
        .map(|seed| seed_to_soil.get(seed))
        .map(|soil| soil_to_fertilizer.get(soil))
        .map(|f| fertilizer_to_water.get(f))
        .map(|water| water_to_light.get(water))
        .map(|light| light_to_temp.get(light))
        .map(|temp| temp_to_humidity.get(temp))
        .map(|humidity| humidity_to_location.get(humidity))
        .collect();

    Ok(location_nums.iter().min().unwrap().to_string())
}

fn parse_line_to_mappings(line: &str, m: &mut Mappings) {
    let mut nums_iter = line
        .trim()
        .split(" ")
        .filter(|num| !num.is_empty())
        .map(|num| num.parse().unwrap());
    let dest_start: u64 = nums_iter.next().unwrap();
    let src_start = nums_iter.next().unwrap();
    let range_len = nums_iter.next().unwrap();

    m.add(Mapping {
        src_start,
        src_end: src_start + range_len,
        dest_start,
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let want = "35";
        match process(input) {
            Ok(r) => assert_eq!(want, r),
            Err(e) => panic!("Error: {}", e),
        }
    }
}
