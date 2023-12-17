
use std::cmp::min;
use std::collections::Bound::{Excluded, Included};
use std::collections::{Bound, HashMap};
use std::iter::once;
use eater_rangeset::{r, Range, range_set, RangeSet};


//https://docs.rs/ranges/0.3.3/ranges/struct.Ranges.html

struct RangeMap {
    dest_start: u64,
    src_start: u64,
    count: u64,
}

impl RangeMap {


}

struct SpecialMap {
    ranges: Vec<RangeMap>,
}

impl SpecialMap {

    fn get(&self, v: &u64) -> u64 {
        for range in &self.ranges {
            if *v >= range.src_start && *v < range.src_start + range.count {
                return (v - range.src_start) + range.dest_start;
            }
        }
        *v
    }

    fn input_range_set(&self) -> RangeSet<u64> {
        let mut acc: RangeSet<u64> = RangeSet::new();

        for rm in &self.ranges {
            let start = rm.src_start;
            let end = rm.src_start + rm.count;

            //acc = acc.union(&range_set![r!(start..end)]);
        }
        acc
    }

    fn map(&self,  in_range: &RangeSet<u64>) -> RangeSet<u64> {
        let input_range = self.input_range_set();
        let intersection = in_range.intersection(&input_range);
        let outside = in_range.difference(&input_range);


        for range in intersection.items() {
            match range.start() {
                Included(x) => println!("i{x}"),
                Excluded(x) => println!("e{x}"),
                //Unbounded => Unbounded,
                _ => {}
            }
        }
        input_range

    }

}

fn main() {


    let lines: Vec<&str> = include_str!("input.txt").lines().into_iter().collect();

    let seeds_str: String = lines[0].chars().skip(7).collect();
    let seeds: Vec<u64> = seeds_str.split(" ").into_iter().map(|s| s.parse::<u64>().unwrap()).collect();

    let seed_to_soil = read_map("seed-to-soil", &lines);
    let soil_to_fertilizer = read_map("soil-to-fertilizer", &lines);
    let fertilizer_to_water = read_map("fertilizer-to-water", &lines);
    let water_to_light = read_map("water-to-light", &lines);
    let light_to_temperature = read_map("light-to-temperature", &lines);
    let temperature_to_humidity = read_map("temperature-to-humidity", &lines);
    let humidity_to_location = read_map("humidity-to-location", &lines);

    let location_num: u64 = seeds.iter()
        .map(|s| seed_to_soil.get(s))
        .map(|s| soil_to_fertilizer.get(&s))
        .map(|s| fertilizer_to_water.get(&s))
        .map(|s| water_to_light.get(&s))
        .map(|s| light_to_temperature.get(&s))
        .map(|s| temperature_to_humidity.get(&s))
        .map(|s| humidity_to_location.get(&s))
        .min()
        .unwrap();
    println!("part1: {location_num}");


    let mut global_min:u64 = 999999999999999999;
    let mut count_x: usize = 0;
    for i in (0..seeds.len()).step_by(2) {
        let start_index = seeds[i];
        let count = seeds[i+1];
        println!("{start_index} {count}");
        for s in start_index..(start_index+count) {
            let mina = once(s)
                .map(|s| seed_to_soil.get(&s))
                .map(|s| soil_to_fertilizer.get(&s))
                .map(|s| fertilizer_to_water.get(&s))
                .map(|s| water_to_light.get(&s))
                .map(|s| light_to_temperature.get(&s))
                .map(|s| temperature_to_humidity.get(&s))
                .map(|s| humidity_to_location.get(&s))
                .min()
                .unwrap();
            global_min = min(global_min, mina);

            count_x += 1;
            if count_x % 100000 == 0 {
                print!("a");
            }
        }
        println!("b");
    }
    println!("part2: {global_min}");
}

fn read_map(name: &str, lines: &Vec<&str>) -> SpecialMap {
    let mut index = 0;
    while true {
        let line = lines[index];
        if line.contains(name) {
            index += 1;
            break;
        }
        index += 1;
    }
    let mut number_lines: Vec<&str> = Vec::new();
    while true {
        if index >= lines.len() {
            break;
        }
        let line = lines[index];
        if line.is_empty() {
            break;
        }
        number_lines.push(line);
        index += 1;
    }


    let mut ranges = Vec::new();
    for number_line in number_lines {
        let split: Vec<u64> = number_line.split(" ").map(|n| n.parse::<u64>().unwrap()).collect();
        let dest_start = split[0];
        let src_start = split[1];
        let count = split[2];
        ranges.push(RangeMap { dest_start: dest_start, src_start: src_start, count: count })
    }
    let mut map: SpecialMap = SpecialMap {
        ranges: ranges,
    };

    map
}

