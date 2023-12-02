use std::{fs::read_to_string, path::PathBuf, str::from_utf8};

#[derive(Default)]
struct Cubes {
    red: usize,
    green: usize,
    blue: usize,
}

fn main() {
    let input = PathBuf::from("input.txt");
    let res_1 = part1(&input);
    println!("{}", res_1);
    let res_2 = part2(&input);
    println!("{}", res_2);
}

fn part1(input: &PathBuf) -> usize {
    let mut sum: usize = 0;
    for line in read_to_string(input).unwrap().lines() {
        let bytes = line.as_bytes();
        let (game_id, game_bytes) = strip_game_id(bytes);
        let rounds = strip_rounds(game_bytes);
        let possible = rounds.iter().all(|round| match round {
            round if round.red > 12 => false,
            round if round.green > 13 => false,
            round if round.blue > 14 => false,
            _ => true,
        });
        if possible {
            sum += game_id;
        }
    }
    sum
}

fn part2(input: &PathBuf) -> usize {
    let mut sum: usize = 0;
    for line in read_to_string(input).unwrap().lines() {
        let bytes = line.as_bytes();
        let (_, game_bytes) = strip_game_id(bytes);
        let rounds = strip_rounds(game_bytes);
        let mut min_bag = Cubes::default();
        rounds.iter().for_each(|round| {
            if round.red > min_bag.red {
                min_bag = Cubes {
                    red: round.red.clone(),
                    ..min_bag
                }
            }
            if round.green > min_bag.green {
                min_bag = Cubes {
                    green: round.green.clone(),
                    ..min_bag
                }
            }
            if round.blue > min_bag.blue {
                min_bag = Cubes {
                    blue: round.blue.clone(),
                    ..min_bag
                }
            }
        });
        sum += min_bag.red * min_bag.blue * min_bag.green;
    }
    sum
}

fn parse_digit(bytes: &[u8]) -> usize {
    from_utf8(bytes).unwrap().parse::<usize>().unwrap()
}

fn strip_game_id(bytes: &[u8]) -> (usize, &[u8]) {
    match bytes {
        [b'G', b'a', b'm', b'e', b' ', byte, b':', tail @ ..] => {
            (parse_digit(&[byte.clone()]), tail)
        }
        [b'G', b'a', b'm', b'e', b' ', byte_1, byte_2, b':', tail @ ..] => {
            (parse_digit(&[byte_1.clone(), byte_2.clone()]), tail)
        }
        [b'G', b'a', b'm', b'e', b' ', byte_1, byte_2, byte_3, b':', tail @ ..] => (
            parse_digit(&[byte_1.clone(), byte_2.clone(), byte_3.clone()]),
            tail,
        ),
        _ => panic!("Unable to fetch game id"),
    }
}

fn strip_rounds(bytes: &[u8]) -> Vec<Cubes> {
    let mut rounds = Vec::new();
    let mut remaining = bytes;
    let mut round = Cubes::default();
    let mut size: usize = 0;
    loop {
        match remaining {
            [] => {
                rounds.push(round);
                break;
            }
            [b';', tail @ ..] => {
                remaining = tail;
                rounds.push(round);
                round = Cubes::default();
            }
            [b',', tail @ ..] => {
                remaining = tail;
            }
            [b' ', byte, b' ', tail @ ..] => {
                size = parse_digit(&[byte.clone()]);
                remaining = tail;
            }
            [b' ', byte_1, byte_2, b' ', tail @ ..] => {
                size = parse_digit(&[byte_1.clone(), byte_2.clone()]);
                remaining = tail;
            }

            [b'r', b'e', b'd', tail @ ..] => {
                remaining = tail;
                round = Cubes {
                    red: size.clone(),
                    ..round
                }
            }
            [b'b', b'l', b'u', b'e', tail @ ..] => {
                remaining = tail;
                round = Cubes {
                    blue: size.clone(),
                    ..round
                }
            }
            [b'g', b'r', b'e', b'e', b'n', tail @ ..] => {
                remaining = tail;
                round = Cubes {
                    green: size.clone(),
                    ..round
                }
            }
            _ => panic!("Unknown symbol!"),
        }
    }
    rounds
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::{part1, part2};

    #[test]
    fn problem_1() {
        let path = PathBuf::from("example.txt");
        let sum = part1(&path);
        assert_eq!(8, sum)
    }
    #[test]
    fn problem_2() {
        let path = PathBuf::from("example.txt");
        let sum = part2(&path);
        assert_eq!(2286, sum)
    }
}
