use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{cmp, io};

pub fn pt1(path: &str) -> Result<i64, io::Error> {
    let file = File::open(path)?;

    let buffered = BufReader::new(file);

    // create an iterator over the lines of the file
    let mut lines_iter = buffered.lines().filter_map(|line_result| line_result.ok());

    let seeds_line = lines_iter.next().unwrap();

    let mut seeds_splits = seeds_line.split(' ');

    seeds_splits.next();

    let seeds: Vec<i64> = seeds_splits.map(|s| str::parse(s).unwrap()).collect();

    let mut map_of_map_builders: HashMap<String, MapBuilder> = HashMap::new();

    let mut current_header = String::from("");

    for line in lines_iter {
        if let Some(c) = line.chars().next() {
            if c.is_digit(10) {
                let mut nums_iter = line.split(' ').map(|s| str::parse(s).unwrap());
                let destination_start = nums_iter.next().unwrap();
                let source_start = nums_iter.next().unwrap();
                let length = nums_iter.next().unwrap();
                let map = map_of_map_builders.get_mut(&current_header).unwrap();
                map.submaps
                    .push(SubMap::new(source_start, destination_start, length));
            } else {
                map_of_map_builders.insert(line.clone(), MapBuilder::new());
                current_header = line;
            }
        }
    }

    let mut map_of_maps: HashMap<String, Map> = HashMap::new();

    for (k, v) in map_of_map_builders {
        map_of_maps.insert(k, v.build());
    }

    let merged_map = seed_location_map(map_of_maps);

    let result = seeds
        .iter()
        .map(|s| merged_map.get_destination(*s))
        .min()
        .unwrap();

    return Ok(result);
}

/// At a high level, the approach is:
/// 1. Merge all the maps into a single seed to location map
/// 2. For each seed range, find the submaps in the map that intersect with the seed range.
/// 3. For each of these submaps, find the location that corresponds to the lower bound of the submap's source range. If the lower bound of the submap's source range is lower than the lower bound of the seed range, use the location corresponding to the location's seed range instead.
/// 4. The answer should be the minimum of these locations.
pub fn pt2(path: &str) -> Result<i64, io::Error> {
    let file = File::open(path)?;

    let buffered = BufReader::new(file);

    // create an iterator over the lines of the file
    let mut lines_iter = buffered.lines().filter_map(|line_result| line_result.ok());

    // assume first line contains seed ranges
    let seeds_line = lines_iter.next().unwrap();

    let mut seeds_splits = seeds_line.split(' ');

    // leave off the "seeds:" part
    seeds_splits.next();

    let seeds: Vec<i64> = seeds_splits.map(|s| str::parse(s).unwrap()).collect();

    let mut map_of_map_builders: HashMap<String, MapBuilder> = HashMap::new();

    let mut current_header = String::from("");

    for line in lines_iter {
        if let Some(c) = line.chars().next() {
            if c.is_digit(10) {
                let mut nums_iter = line.split(' ').map(|s| str::parse(s).unwrap());
                let destination_start = nums_iter.next().unwrap();
                let source_start = nums_iter.next().unwrap();
                let length = nums_iter.next().unwrap();
                let map = map_of_map_builders.get_mut(&current_header).unwrap();
                map.submaps
                    .push(SubMap::new(source_start, destination_start, length));
            } else {
                map_of_map_builders.insert(line.clone(), MapBuilder::new());
                current_header = line;
            }
        }
    }

    let mut map_of_maps: HashMap<String, Map> = HashMap::new();

    for (k, v) in map_of_map_builders {
        map_of_maps.insert(k, v.build());
    }

    let merged_map = seed_location_map(map_of_maps);

    println!("merged_map num submaps {}", merged_map.submaps.len());

    // assumes there are an even number of seed numbers
    let seed_ranges = seeds.chunks_exact(2);

    let result = seed_ranges
        .map(|seed_range| {
            let &start = seed_range.get(0).unwrap();
            let &length = seed_range.get(1).unwrap();
            let end = start + length;

            merged_map
                .intersecting_submaps(start, end)
                .map(|sm| cmp::max(start, sm.source_start) + sm.destination_difference)
                .min()
        })
        .filter_map(|x| x)
        .min()
        .unwrap();

    return Ok(result);
}

/// A "submap" corresponds to a line in the map input
#[derive(PartialEq, Debug)]
struct SubMap {
    /// the start of the source range (inclusive)
    source_start: i64,

    /// the end of the source range (exclusive)
    source_end: i64,

    /// the number that can be added to the source to obtain the destination
    destination_difference: i64,
}

impl SubMap {
    fn new(source_start: i64, destination_start: i64, length: i64) -> Self {
        SubMap {
            source_start,
            source_end: source_start + length,
            destination_difference: destination_start - source_start,
        }
    }

    fn get_destination(&self, source: i64) -> Option<i64> {
        if source < self.source_start || source >= self.source_end {
            return None;
        } else {
            return Some(source + self.destination_difference);
        }
    }
}

/// An intermediate representation of a Map. Build up this struct first and then call the build method to get a Map.
#[derive(PartialEq, Debug)]
struct MapBuilder {
    submaps: Vec<SubMap>,
}

impl MapBuilder {
    fn new() -> Self {
        Self {
            submaps: Vec::new(),
        }
    }

    fn build(mut self) -> Map {
        self.submaps
            .sort_by(|a, b| a.source_start.cmp(&b.source_start));

        let mut map = Map {
            submaps: Vec::new(),
        };

        let first_submap = self.submaps.get(0).unwrap();

        map.submaps.push(SubMap {
            source_start: i64::MIN,
            source_end: first_submap.source_start,
            destination_difference: 0,
        });

        for submap in self.submaps {
            if let Some(last_submap) = map.submaps.last() {
                // check for gap
                if last_submap.source_end < submap.source_start {
                    map.submaps.push(SubMap {
                        source_start: last_submap.source_end,
                        source_end: submap.source_start,
                        destination_difference: 0,
                    })
                }
            }

            map.submaps.push(submap);
        }

        map.submaps.push(SubMap {
            source_start: map.submaps.last().unwrap().source_end,
            source_end: i64::MAX,
            destination_difference: 0,
        });

        map
    }
}

#[derive(PartialEq, Debug)]
struct Map {
    submaps: Vec<SubMap>,
}

impl Map {
    fn get_destination(&self, source: i64) -> i64 {
        self.submaps
            .iter()
            .map(|s| s.get_destination(source))
            .find_map(|d| d)
            .unwrap_or(source)
    }

    /// takes a range start and range end and returns all submaps that intersect with that range.
    fn intersecting_submaps(
        &self,
        range_start: i64,
        range_end: i64,
    ) -> impl Iterator<Item = &SubMap> {
        self.submaps
            .iter()
            .filter(move |sm| sm.source_start >= range_start && sm.source_start < range_end)
    }

    /// takes two maps A -> B and B -> C and merges into a single A -> C map
    fn merge(&self, other: &Self) -> Self {
        let mut result = Self {
            submaps: Vec::new(),
        };

        for submap in &self.submaps {
            'inner: loop {
                // continue from the end of the last submap added to result
                let start = match result.submaps.last() {
                    Some(sm) => sm.source_end,
                    None => submap.source_start,
                };

                let destination_start = start.saturating_add(submap.destination_difference);

                // find other submap that corresponds to current submap destination start
                let other_submap = other
                    .seek(destination_start)
                    .expect(&format!("failed to find {} in other map", start));

                // shift other submap source end to correspond with current submap source
                let source_end_other = other_submap
                    .source_end
                    .saturating_sub(submap.destination_difference);

                let end = cmp::min(submap.source_end, source_end_other);

                let dd = submap.destination_difference + other_submap.destination_difference;

                result.submaps.push(SubMap {
                    source_start: start,
                    source_end: end,
                    destination_difference: dd,
                });

                // if current submap source end was used, move on to next submap
                if end == submap.source_end {
                    break 'inner;
                }
            }
        }

        result
    }

    /// returns the submap that contains source
    fn seek(&self, source: i64) -> Option<&SubMap> {
        for submap in &self.submaps {
            if source >= submap.source_start && source < submap.source_end {
                return Some(submap);
            }
        }
        None
    }
}

/// takes a map of maps, returns a seed -> location map
fn seed_location_map(map_of_maps: HashMap<String, Map>) -> Map {
    let seed_to_soil_map = map_of_maps.get("seed-to-soil map:").unwrap();
    let soil_to_fertilizer_map = map_of_maps.get("soil-to-fertilizer map:").unwrap();
    let fertilizer_to_water_map = map_of_maps.get("fertilizer-to-water map:").unwrap();
    let water_to_light_map = map_of_maps.get("water-to-light map:").unwrap();
    let light_to_temperature_map = map_of_maps.get("light-to-temperature map:").unwrap();
    let temperature_to_humidity_map = map_of_maps.get("temperature-to-humidity map:").unwrap();
    let humidity_to_location_map = map_of_maps.get("humidity-to-location map:").unwrap();

    seed_to_soil_map
        .merge(soil_to_fertilizer_map)
        .merge(fertilizer_to_water_map)
        .merge(water_to_light_map)
        .merge(light_to_temperature_map)
        .merge(temperature_to_humidity_map)
        .merge(humidity_to_location_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_works() {
        assert_eq!(pt1("test_input").unwrap(), 35);
        assert_eq!(pt1("input").unwrap(), 261668924);
    }

    #[test]
    fn pt2_works() {
        assert_eq!(pt2("test_input").unwrap(), 46);
        assert_eq!(pt2("input").unwrap(), 24261545);
        assert_eq!(pt2("tonys_input").unwrap(), 37806486);
        assert_eq!(pt2("tims_input").unwrap(), 69841803);
        assert_eq!(pt2("diggseys_input").unwrap(), 20283860);
    }
}
