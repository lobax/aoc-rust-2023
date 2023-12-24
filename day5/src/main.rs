use std::{
    fs::read_to_string, 
    path::PathBuf};


struct Almanac {
    dest_maps: Vec<Box<dyn Fn(&isize) -> Option<isize>>>
}

impl Almanac { 
    fn set_dest(&mut self, destination: isize, source: isize, range: isize) { 
        let dest_map = Box::new(move |s: &isize| match s {
            s if (source..(source+range)).contains(&s) => {
                Some(destination + (s-source))
            },
            _ => None
        });
        self.dest_maps.push(dest_map);
    }

    fn get_dest(&self, source: &isize) -> isize { 
        self.dest_maps
            .iter()
            .filter_map(|f| f(source))
            .last()
            .unwrap_or(source.clone())
    }
}

fn main() {
    let input = PathBuf::from("input.txt");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}


fn part1(input: &PathBuf) -> isize { 
    let reader = read_to_string(input).unwrap();
    let mut lines = reader.lines();
    let seeds = parse_seeds(lines.next().unwrap());

    let _ = lines.next();

    let seed_to_soil            = parse_almanac(&mut lines);
    let soil_to_fertilizer      = parse_almanac(&mut lines);
    let fertilizer_to_water     = parse_almanac(&mut lines);
    let water_to_light          = parse_almanac(&mut lines);
    let light_to_temperature    = parse_almanac(&mut lines);
    let temperature_to_humidity = parse_almanac(&mut lines);
    let humidity_to_location    = parse_almanac(&mut lines);
    
    seeds.iter()
        .map(|s| seed_to_soil.get_dest(s))
        .map(|s| soil_to_fertilizer.get_dest(&s))
        .map(|s| fertilizer_to_water.get_dest(&s))
        .map(|s| water_to_light.get_dest(&s))
        .map(|s| light_to_temperature.get_dest(&s))
        .map(|s| temperature_to_humidity.get_dest(&s))
        .map(|s| humidity_to_location.get_dest(&s))
        .min()
        .unwrap()
}


fn parse_seeds(seeds_str: &str) -> Vec<isize> { 
   let seeds = seeds_str.split(": ").last().unwrap();
   seeds
       .split_whitespace()
       .filter_map(|s| s.parse::<isize>().ok())
       .collect()
}


fn parse_almanac<'a>(iter: &mut impl Iterator<Item=&'a str>) -> Almanac { 
    let mut almanac: Almanac = Almanac {
        dest_maps: Vec::new()
    };

    let _ = iter.next();
    while let Some(line) = iter.next() { 
        if line == "" { 
            break;
        }
        let map: Vec<isize> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<isize>().ok())
            .collect();
        almanac.set_dest(map[0], map[1], map[2])
    }
    almanac
}


#[derive(Clone, PartialEq, Eq, Debug)]
struct Range { 
    start: isize,
    end: isize,
}

impl Range { 
    fn contains(&self, number: isize) -> bool {
        (self.start..=self.end).contains(&number)
    }

    fn complement(&self, range: &Range) -> Vec<Range> { 
        let mut complement = Vec::new();
        if self.start < range.start { 
            complement.push( Range { 
                start: self.start,
                end: [self.end, range.start -1].iter().min().unwrap().clone(),
            } )
        }
        if self.end > range.end { 
            complement.push( Range { 
                start: [self.start, range.end + 1].iter().max().unwrap().clone(),
                end: self.end, 
            } )
        }
        complement
    }

    fn intersection(&self, range: &Range) -> Option<Range> { 
        match (self, range) { 
            (a, b) if a.start > b.end => None,
            (a, b) if b.start > a.end => None,
            (a, b) => Some(Range { 
                start:  [a.start, b.start].iter().max().unwrap().clone(),
                end:    [a.end, b.end].iter().min().unwrap().clone(),
            })
        }
    }
}

#[derive(Debug)]
struct RangeMap {
    source: Range,
    destination: Range,
}

impl RangeMap { 
    fn offset(&self) -> isize { 
        self.source.start - self.destination.start
    }

    fn map_complement(&self, range: &Range) -> Option<Vec<Range>> { 
        match range.complement(&self.source) { 
            complement if complement.len() > 0 => Some(complement),
            _ => None
        }
    }

    fn map_intersection(&self, range: &Range) -> Option<Range> { 
        self.source.intersection(&range).map( |intersection| { 
                let mapped = Range { 
                    start: intersection.start - self.offset(),
                    end: intersection.end - self.offset(),
                };
                mapped
        })
    }
}

struct AlmanacRanges { 
    range_maps: Vec<RangeMap>
}

impl AlmanacRanges { 
    fn map(&self, ranges: impl IntoIterator<Item=Range>) -> Vec<Range> { 
        let mut res = Vec::new();
        let mut complements = Vec::new();
        let mut intersections = Vec::new();
        for range in ranges { 
            // println!("Range: {:?}", range);
            let mut _complements = Vec::new();
            for range_map in self.range_maps.iter() { 
                // println!("  RangeMap: {:?}", range_map);
                let _complement = range_map.map_complement(&range); 
                _complements.push(_complement);
                if let Some(intersection) = range_map.map_intersection(&range){ 
                    // println!("  intersection: {:?}", intersection);
                    intersections.push(intersection);
                }
            }

            let _complement: Option<Range> = _complements.into_iter()
                .flat_map(|c| match c {
                    Some(v) => v.iter().map(|c| Some(c.clone())).collect(),
                    None => vec!(None),
                })
                .reduce(|a, b| a?.intersection(&b?))
                .flatten();
            if let Some(complement) = _complement { 
                // println!("  complement: {:?}", complement);
                complements.push(complement);
            }
        }
        res.append(&mut complements);
        res.append(&mut intersections);
        res
    }
}

fn part2(input: &PathBuf) -> isize { 
    let reader = read_to_string(input).unwrap();
    let mut lines = reader.lines();
    let seed_ranges = parse_seed_ranges(lines.next().unwrap());

    let _ = lines.next();

    let seed_to_soil            = parse_almanac_ranges(&mut lines);
    let soil_to_fertilizer      = parse_almanac_ranges(&mut lines);
    let fertilizer_to_water     = parse_almanac_ranges(&mut lines);
    let water_to_light          = parse_almanac_ranges(&mut lines);
    let light_to_temperature    = parse_almanac_ranges(&mut lines);
    let temperature_to_humidity = parse_almanac_ranges(&mut lines);
    let humidity_to_location    = parse_almanac_ranges(&mut lines);

    let soils           = seed_to_soil.map(seed_ranges);
    let fertilizers     = soil_to_fertilizer.map(soils);
    let waters          = fertilizer_to_water.map(fertilizers);
    let lights          = water_to_light.map(waters);
    let temperatures    = light_to_temperature.map(lights);
    let humiditys       = temperature_to_humidity.map(temperatures);
    let mut locations   = humidity_to_location.map(humiditys);

    locations.sort_by(|a, b| a.start.cmp(&b.start));
    locations[0].start
}

fn parse_seed_ranges(seeds_str: &str) -> Vec<Range> { 
    let mut seeds = seeds_str.split(": ").last().unwrap().split_whitespace();
    let mut ranges = Vec::new();
    while let Some(_start) = seeds.next() { 
        let start = _start.parse::<isize>().unwrap();
        let size = seeds.next().unwrap().parse::<isize>().unwrap(); 
        ranges.push(Range { 
           start, 
           end: (start + size - 1), 
        });
    }
    ranges
}

fn parse_almanac_ranges<'a>(iter: &mut impl Iterator<Item=&'a str>) -> AlmanacRanges {
    let _ = iter.next();
    let mut range_maps= Vec::new();
    while let Some(line) = iter.next() {
        if line == "" { 
            break;
        }
        let map: Vec<isize> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<isize>().ok())
            .collect();
        let source = Range { 
            start: map[1], 
            end: map[1] + map[2] - 1
        };
        let destination = Range { 
            start: map[0], 
            end: map[0] + map[2] - 1
        };
        range_maps.push( RangeMap { 
            source, 
            destination
        } )
    }
    AlmanacRanges { 
        range_maps
    }
}

#[cfg(test)]
mod test { 
    use std::path::PathBuf;
    use crate::*;

    #[test]
    fn problem1() {
        let path = PathBuf::from("example.txt");
        let sum = part1(&path);
        assert_eq!(35, sum);
    }

    #[test]
    fn problem2() {
        let path = PathBuf::from("example.txt");
        let sum = part2(&path);
        assert_eq!(46, sum);
    }

    #[test]
    fn problem2_trace() {
        let path = PathBuf::from("example.txt");
        let reader = read_to_string(&path).unwrap();
        let mut lines = reader.lines();
        let seed_ranges = vec!(Range {start: 82, end: 82}); 
        let _ = lines.next();
        let _ = lines.next();

        let seed_to_soil            = parse_almanac_ranges(&mut lines);
        let soil_to_fertilizer      = parse_almanac_ranges(&mut lines);
        let fertilizer_to_water     = parse_almanac_ranges(&mut lines);
        let water_to_light          = parse_almanac_ranges(&mut lines);
        let light_to_temperature    = parse_almanac_ranges(&mut lines);
        let temperature_to_humidity = parse_almanac_ranges(&mut lines);
        let humidity_to_location    = parse_almanac_ranges(&mut lines);

        let soils           = seed_to_soil.map(seed_ranges);
        assert_eq!(soils[0], Range {start: 84, end: 84});
        let fertilizers     = soil_to_fertilizer.map(soils);
        assert_eq!(fertilizers[0], Range {start: 84, end: 84});
        let waters          = fertilizer_to_water.map(fertilizers);
        assert_eq!(waters[0], Range {start: 84, end: 84});
        let lights          = water_to_light.map(waters);
        assert_eq!(lights[0], Range {start: 77, end: 77});
        let temperatures    = light_to_temperature.map(lights);
        assert_eq!(temperatures[0], Range {start: 45, end: 45});
        let humiditys       = temperature_to_humidity.map(temperatures);
        assert_eq!(humiditys[0], Range {start: 46, end: 46});
        let locations       = humidity_to_location.map(humiditys);
        assert_eq!(locations[0], Range {start: 46, end: 46})
    }

    #[test]
    fn test_range_complement() { 
        let a = Range { start: 0, end: 4 };
        let b = Range { start: 1, end: 2 };
        let c = a.complement(&b);
        assert_eq!(c.len(), 2);
        assert_eq!(c[0], Range { start: 0, end: 0});
        assert_eq!(c[1], Range { start: 3, end: 4});
    }

    #[test]
    fn test_range_intersection() { 
        let a = Range { start: 1, end: 4 };
        let b = Range { start: 0, end: 2 };
        let c = a.intersection(&b);
        assert!(c.is_some());
        assert_eq!(c.unwrap(), Range { start: 1, end: 2});
    }

    #[test]
    fn test_range_map() { 
        let ranges = vec!(Range { start: 5, end: 10 });
        let map = AlmanacRanges {
            range_maps: vec!(RangeMap { 
                source: Range { 
                    start: 7,
                  end: 9
                },
                destination: Range { 
                   start: 2,
                    end: 4
             }
            })
        };
        let mut c = map.map(ranges);
        c.sort_by(|a, b| a.start.cmp(&b.start));

        assert_eq!(c[0], Range { start: 2, end: 4});
    }

    #[test]
    fn test_seed_ranges() {
        let ranges = "seeds: 79 14 55 13";
        let ranges = parse_seed_ranges(&ranges);

        assert!(ranges[0].contains(79));
        assert!(ranges[0].contains(80));
        assert!(ranges[0].contains(91));
        assert!(ranges[0].contains(92));
        assert!(!ranges[0].contains(93));

        assert!(ranges[1].contains(55));
        assert!(ranges[1].contains(56));
        assert!(ranges[1].contains(66));
        assert!(ranges[1].contains(67));
        assert!(!ranges[1].contains(68));
    }

    #[test]
    fn test_map_ranges() {
        let seed_ranges = "seeds: 79 14 55 13";
        let seed_ranges = parse_seed_ranges(&seed_ranges);
        println!("seed ranges: {:?}", seed_ranges);
        let mut range_map = vec!("seed-to-soil map:", "50 98 2", "52 50 48").into_iter();
        let seed_to_soil = parse_almanac_ranges(&mut range_map);

        let mut ranges: Vec<Range> = seed_to_soil.map(seed_ranges); 
        ranges.sort_by(|a, b| a.start.cmp(&b.start));
        println!("final ranges: {:?}", ranges);
        assert_eq!(ranges[0], Range{start: 57, end: 69});
        assert_eq!(ranges[0], Range{start: 57, end: 69});
    }
}
