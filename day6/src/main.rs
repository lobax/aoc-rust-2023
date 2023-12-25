use std::{fs::read_to_string, path::PathBuf, str::from_utf8};

fn main() {
    let input = PathBuf::from("input.txt");
    let res_1 = part1(&input);
    println!("{}", res_1);
    let res_2 = part2(&input);
    println!("{}", res_2);
}

#[derive(Default, Debug)]
struct Race { 
    time: usize, 
    distance: usize,
}

fn parse(input: &PathBuf) -> Vec<Race> { 
    let reader = read_to_string(input).unwrap();
    let mut lines = reader.lines();
    let mut time_iter = lines.next().unwrap().split_whitespace();
    let mut distance_iter = lines.next().unwrap().split_whitespace();

    let _ = (time_iter.next(), distance_iter.next());

    let mut races = Vec::new();
    while let (Some(time), Some(distance)) = (time_iter.next(), distance_iter.next()) { 
        races.push(Race { 
            time: time.parse::<usize>().unwrap(),
            distance: distance.parse::<usize>().unwrap(),
        });
    }

    races
}

fn reduced_quadratic(p: f64, q: f64) -> (f64, f64) { 
    let right = ((p/2_f64).powi(2) -q).sqrt();
    let left = -p / 2_f64;
    ((left - right), (left + right))
}

fn combinations_that_go_farther(race: &Race) -> usize { 
    println!("Race: {:?}", race);
    let p = -(race.time as f64);
    let q = race.distance as f64;
    let (a, b) = reduced_quadratic(p, q);
    // For "perfect" results we need to nudge things in the right direction
    let res = ((b - 0.001_f64).floor() - (a + 0.0001_f64).ceil()) as usize;
    res + 1 // Off by one error 
}

fn part1(path: &PathBuf) -> usize { 
    let races = parse(path);
    races.iter()
        .map(|r| { 
            let res = combinations_that_go_farther(r);
            println!("{}", res);
            res
        })
        .product()
}

fn parse2(input: &PathBuf) -> Race { 
    let reader = read_to_string(input).unwrap();
    let mut lines = reader.lines();
    let mut time_iter = lines.next().unwrap().split_whitespace();
    let mut distance_iter = lines.next().unwrap().split_whitespace();

    let _ = (time_iter.next(), distance_iter.next());

    let mut time = String::from("");
    let mut distance = String::from("");
    while let (Some(_time), Some(_distance)) = (time_iter.next(), distance_iter.next()) { 
        time.push_str(_time);
        distance.push_str(_distance);
    }

    Race { 
        time: time.parse::<usize>().unwrap(),
        distance: distance.parse::<usize>().unwrap(),
    }
}


fn part2(path: &PathBuf) -> usize { 
    let race = parse2(path);
    combinations_that_go_farther(&race)
}


#[cfg(test)]
mod test { 
    use std::path::PathBuf;
    use crate::*;

    #[test]
    fn test_parse() {
        let path = PathBuf::from("example.txt");
        let races = parse(&path);

        assert_eq!(races[0].time, 7);
        assert_eq!(races[0].distance, 9);
        assert_eq!(races[1].time, 15);
        assert_eq!(races[1].distance, 40);
        assert_eq!(races[2].time, 30);
        assert_eq!(races[2].distance, 200);
    } 

    #[test]
    fn part1_test() { 
        let path = PathBuf::from("example.txt");
        let prod = part1(&path);
        assert_eq!(288, prod);
    }

    #[test]
    fn part2_test() { 
        let path = PathBuf::from("example.txt");
        let res = part2(&path);
        assert_eq!(71503, res);
    }
}
