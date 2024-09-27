use std::ops::Range;

use nom::{
    bytes::complete::tag,
    character::complete,
    IResult,
};

#[derive(Debug)]
struct Map {
    source: u64,
    target: u64,
    range: u64,
}


impl Map {

    fn inside_range(&self, value: u64) -> bool {
        value >= self.source && value < self.source + self.range
    }

    fn convert(&self, value: u64) -> Option<u64> {
        if self.inside_range(value) {
            // println!("{} is inside range {}-{}", value, self.source, self.source + self.range);
            // println!("{} -> {}", value, self.target + (value - self.source));
            Some(self.target + (value - self.source))
        } else {
            None
        }
    }

    fn start(&self) -> u64 {
        self.source
    }

    fn end(&self) -> u64 {
        self.source + self.range
    }

    fn ovelap_from_range(&self, range: Range<u64>) -> Option<(Range<u64>, Vec<Range<u64>>, &str)>{
        
        if range.start >= self.start() && range.start < self.end() && range.end > self.start() && range.end <= self.end() {
            // println!("{}-{} fully within {}-{}", range.start, range.end, self.source, self.source + self.range);
            //Start and end of range is inside the map
            //Return the remapped range with no remainder
            let start = self.target + (range.start - self.source);
            let end = self.target + (range.end - self.source);

            Some((start..end, vec![], "Fully within"))
        } else if range.start >= self.start() && range.start < self.end() {
            // println!("{}-{} starts within {}-{}", range.start, range.end, self.source, self.source + self.range);
            //Start of range is inside the map
            //Return the remapped range and the unmapped remainder
            let start = self.target + (range.start - self.source);
            let end = self.target + self.range;
            let remainder = (self.source + self.range)..range.end;
            // println!("Maps to {}-{} with remainder {:?}", start, end, remainder);
            Some((start..end, vec![remainder], "Start within"))
        } else if range.end > self.start() && range.end < self.end() {
            // println!("{}-{} ends within {}-{}", range.start, range.end, self.source, self.source + self.range);
            // End of range is inside the map
            // Return the remapped range with the remainder
            let start = self.target;
            let end = self.target + (range.end - self.source);
            let remainder = range.start..(self.source);
            // println!("Maps to {}-{} with remainder {:?}", start, end, remainder);
            Some((start..end, vec![remainder], "End within"))
        } else if range.start < self.start() && range.end >= self.end() { 
            // println!("{}-{} covers {}-{}", range.start, range.end, self.source, self.source + self.range);
            // Range covers the whole map
            let start = self.target; 
            let end = self.target + self.range;
            let left_remainder = range.start..self.source;
            let right_remainder = (self.source + self.range)..range.end;
            let mut remainder = vec![];
            if left_remainder.start < left_remainder.end {
                remainder.push(left_remainder);
            }
            if right_remainder.start < right_remainder.end {
                remainder.push(right_remainder);
            }
            Some((start..end, remainder, "Covers"))
        } else {
            None
        }
    }
}


fn convert_ranges(maps: &Vec<Map>, ranges: Vec<Range<u64>>) -> Vec<Range<u64>> {
    let mut unconverted_ranges = ranges;
    let mut converted_ranges: Vec<Range<u64>> = vec![];
    // maps.iter().for_each(|m| println!("{} - {}", m.source, m.source + m.range));
    while let Some(range) = unconverted_ranges.pop() {
        let mut converted_range = None;
        for map in maps {
            if let Some((new_range, remainder, desc)) = map.ovelap_from_range(range.clone()) {
                assert!(new_range.start < new_range.end, "Invalid range: {:?}, desc: {}", new_range, desc);
                converted_range = Some(new_range);
                remainder.iter().enumerate().for_each(|(i, r)| assert!(r.start < r.end, "Invalid remainder {} range: {:?}, desc: {}", i, r, desc));
                unconverted_ranges.extend(remainder);
                break;
            }
        }
        if let Some(converted_range) = converted_range {
            converted_ranges.push(converted_range);
        } else {
            converted_ranges.push(range);
        }

    }
    converted_ranges
}

#[derive(Debug)]
struct Maps {
    seed_to_soil: Vec<Map>,
    soil_to_fertilizer: Vec<Map>,
    fertilizer_to_water: Vec<Map>,
    water_to_light: Vec<Map>,
    light_to_temperature: Vec<Map>,
    temperature_to_humidity: Vec<Map>,
    humidity_to_location: Vec<Map>,
}

fn parse_map(input: &str) -> IResult<&str, Map> {
    let (input, target) = complete::u64(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, source) = complete::u64(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, range) = complete::u64(input)?;
    Ok((
        input,
        Map {
            source,
            target,
            range,
        },
    ))
}

fn parse_input(input: &str) -> IResult<&str, (Vec<u64>, Maps)> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, seeds) = nom::multi::separated_list1(tag(" "), complete::u64)(input)?;
    let (input, _) = tag("\n\nseed-to-soil map:\n")(input)?;
    let (input, seed_to_soil) = nom::multi::separated_list1(tag("\n"), parse_map)(input)?;
    let (input, _) = tag("\n\nsoil-to-fertilizer map:\n")(input)?;
    let (input, soil_to_fertilizer) = nom::multi::separated_list1(tag("\n"), parse_map)(input)?;
    let (input, _) = tag("\n\nfertilizer-to-water map:\n")(input)?;
    let (input, fertilizer_to_water) = nom::multi::separated_list1(tag("\n"), parse_map)(input)?;
    let (input, _) = tag("\n\nwater-to-light map:\n")(input)?;
    let (input, water_to_light) = nom::multi::separated_list1(tag("\n"), parse_map)(input)?;
    let (input, _) = tag("\n\nlight-to-temperature map:\n")(input)?;
    let (input, light_to_temperature) = nom::multi::separated_list1(tag("\n"), parse_map)(input)?;
    let (input, _) = tag("\n\ntemperature-to-humidity map:\n")(input)?;
    let (input, temperature_to_humidity) =
        nom::multi::separated_list1(tag("\n"), parse_map)(input)?;
    let (input, _) = tag("\n\nhumidity-to-location map:\n")(input)?;
    let (input, humidity_to_location) = nom::multi::separated_list1(tag("\n"), parse_map)(input)?;

    Ok((
        input,
        (
            seeds,
            Maps {
                seed_to_soil,
                soil_to_fertilizer,
                fertilizer_to_water,
                water_to_light,
                light_to_temperature,
                temperature_to_humidity,
                humidity_to_location,
            },
        ),
    ))
}

pub fn process_part_1(input: &str) -> u64 {
    let (_, (seeds, maps)) = parse_input(input).unwrap();
    seeds.iter().map(|s| {
        println!("Seed: {}", s);
        let soil = maps.seed_to_soil.iter().find_map(|m| m.convert(*s)).unwrap_or(*s);
        println!("Soil: {}", soil);
        let fertilizer = maps.soil_to_fertilizer.iter().find_map(|m| m.convert(soil)).unwrap_or(soil);
        println!("Fertilizer: {}", fertilizer);
        let water = maps.fertilizer_to_water.iter().find_map(|m| m.convert(fertilizer)).unwrap_or(fertilizer);
        println!("Water: {}", water);
        let light = maps.water_to_light.iter().find_map(|m| m.convert(water)).unwrap_or(water);
        println!("Light: {}", light);
        let temperature = maps.light_to_temperature.iter().find_map(|m| m.convert(light)).unwrap_or(light);
        println!("Temperature: {}", temperature);
        let humidity = maps.temperature_to_humidity.iter().find_map(|m| m.convert(temperature)).unwrap_or(temperature);
        println!("Humidity: {}", humidity);
        let location = maps.humidity_to_location.iter().find_map(|m| m.convert(humidity)).unwrap_or(humidity);
        println!("Location: {}", location);
        location
    }).min().unwrap()
}

fn length_of_range_list(ranges: &Vec<Range<u64>>) -> u64 {
    ranges.iter().map(|r| r.end - r.start).sum()
}

pub fn process_part_2(input: &str) -> u64 {
    let (_, (seeds, maps)) = parse_input(input).unwrap();
    let seeds = seeds.chunks(2).map(|w| {
        let range = w[0]..(w[0] + w[1]);
        range
         }).collect::<Vec<Range<u64>>>();
    // println!("Seeds: {:?}: {}", seeds, length_of_range_list(&seeds));
    let soil = convert_ranges(&maps.seed_to_soil, seeds);
    // println!("Soil: {:?}: {}", soil, length_of_range_list(&soil));
    let fertilizer = convert_ranges(&maps.soil_to_fertilizer, soil);
    // println!("Fertilizer: {:?}", fertilizer);
    let water = convert_ranges(&maps.fertilizer_to_water, fertilizer);
    // println!("Water: {:?}", water);
    let light = convert_ranges(&maps.water_to_light, water);
    // println!("Light: {:?}", light);
    let temperature = convert_ranges(&maps.light_to_temperature, light);
    // println!("Temperature: {:?}", temperature);
    let humidity = convert_ranges(&maps.temperature_to_humidity, temperature);
    // println!("Humidity: {:?}", humidity);
    let mut locations = convert_ranges(&maps.humidity_to_location, humidity);
    locations.sort_by_key(|r| r.start);
    // println!("Locations: {:?}", locations);
    locations.iter().map(|r| r.start).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PUZZLE_INPUT: &str = "seeds: 79 14 55 13

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

    #[test]
    fn test_part_1() {
        assert_eq!(process_part_1(EXAMPLE_PUZZLE_INPUT), 35, "Failed example 1");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(process_part_2(EXAMPLE_PUZZLE_INPUT), 46, "Failed example 2");
    }


    #[test]
    fn test_conversions() {
        //Fully within
        let maps = vec![Map {
            source: 0,
            target: 30,
            range: 10,
        }];
        let ranges = vec![9..10, 10..20, 20..30];
        let mut converted_ranges = convert_ranges(&maps, ranges);
        converted_ranges.sort_by_key(|r| r.start);
        assert_eq!(converted_ranges, vec![10..20, 20..30, 39..40]);
        
        //Start within
        let maps = vec![Map {
            source: 0,
            target: 30,
            range: 10,
        }];
        let ranges = vec![5..15, 20..30];
        let mut converted_ranges = convert_ranges(&maps, ranges);
        converted_ranges.sort_by_key(|r| r.start);
        assert_eq!(converted_ranges, vec![10..15, 20..30, 35..40]);

        //End within
        let maps = vec![Map {
            source: 5,
            target: 30,
            range: 10,
        }];
        let ranges = vec![0..10, 20..30];
        let mut converted_ranges = convert_ranges(&maps, ranges);
        converted_ranges.sort_by_key(|r| r.start);
        assert_eq!(converted_ranges, vec![0..5, 20..30, 30..35]);

        //Covering
        let maps = vec![Map {
            source: 5,
            target: 30,
            range: 10,
        }];
        let ranges = vec![0..20];
        let mut converted_ranges = convert_ranges(&maps, ranges);
        converted_ranges.sort_by_key(|r| r.start);
        assert_eq!(converted_ranges, vec![0..5, 15..20, 30..40]);

    }
}
