use std::{cmp::Ordering, fs::read_to_string, path::PathBuf};

pub fn part1(path: &PathBuf) -> usize { 
    let reader = read_to_string(path).unwrap();
    let mut hands: Vec<Hand> = reader
        .lines()
        .map(Hand::new)
        .collect();

    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i+1) * hand.bid)
        .sum()
}

#[derive(Debug, Eq, PartialEq)]
enum Type {
    FiveOfAKind(usize),
    FourOfAKind(usize),
    FullHouse(usize, usize),
    ThreeOfAKind(usize),
    TwoPair(usize, usize),
    OnePair(usize),
    HighCard(usize),
    NoValue, 
}

impl Type { 
    fn rank(&self) -> usize { 
        match self { 
            Type::FiveOfAKind(_)    => 7,
            Type::FourOfAKind(_)    => 6,
            Type::FullHouse(_, _)   => 5,
            Type::ThreeOfAKind(_)   => 4,
            Type::TwoPair(_, _)     => 3,
            Type::OnePair(_)        => 2,
            Type::HighCard(_)       => 1,
            Type::NoValue           => 0,
        }
    }
}

impl PartialOrd for Type { 
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { 
        self.rank().partial_cmp(&other.rank())
    }
}

impl Ord for Type { 
    fn cmp(&self, other: &Self) -> Ordering { 
        self.rank().cmp(&other.rank())
    }
}

fn get_type(hand: &str) -> Type { 
    let mut buf = [0_u8; 15];
    _get_type(hand.as_bytes(), &mut buf).expect("Unable to fetch type")
}

fn _get_type(hand: &[u8], vals: &mut [u8]) -> Option<Type> { 
    match hand { 
        [b'A', tail @ ..] => { 
            vals[14] += 1;
            _get_type(tail, vals)
        },
        [b'K', tail @ ..] => { 
            vals[13] += 1;
            _get_type(tail, vals)
        },
        [b'Q', tail @ ..] => { 
            vals[12] += 1;
            _get_type(tail, vals)
        },
        [b'J', tail @ ..] => { 
            vals[11] += 1;
            _get_type(tail, vals)
        },
        [b'T', tail @ ..] => { 
            vals[10] += 1;
            _get_type(tail, vals)
        },
        [b, tail @ ..] if b - 48_u8 <= 9 && b -48_u8 > 0  => { 
            let i: usize = b.clone() as usize - 48;
            vals[i] += 1;
            _get_type(tail, vals)
        },
        [] => {
            let mut hand_type = Type::NoValue;
            for i in 0..vals.len() { 
                match (vals[i], &hand_type) { 
                    (1, Type::NoValue)          => hand_type = Type::HighCard(i),
                    (2, t) if t.rank() < 2      => hand_type = Type::OnePair(i),
                    (2, Type::OnePair(j))       => hand_type = Type::TwoPair(i, j.clone()),
                    (2, Type::ThreeOfAKind(j))  => hand_type = Type::FullHouse(j.clone(), i),
                    (3, t) if t.rank() < 2      => hand_type = Type::ThreeOfAKind(i),
                    (3, Type::OnePair(j))       => hand_type = Type::FullHouse(i, j.clone()),
                    (4, t) if t.rank() < 6      => hand_type = Type::FourOfAKind(i),
                    (5, _)                      => hand_type = Type::FiveOfAKind(i),
                    _                           => continue,
                }
            }
            Some(hand_type)
        }
        _ => None
    }
}

fn get_val(val: &str) -> usize { 
    let mut res = 0;
    for (i, byte) in val.as_bytes().iter().enumerate() { 
        let val = match byte { 
            b'A' => 14,
            b'K' => 13,  
            b'Q' => 12,  
            b'J' => 11,  
            b'T' => 10,  
            b if b - 48_u8 <= 9 && b - 48_u8 > 0  => (b - 48_u8) as usize,
            b => panic!("Unsupported byte {}", b)
        };
        res += val << (4 * (5-i));
    }
    res
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    val: usize,
    bid: usize,
    hand_type: Type,
}

impl Hand { 
    fn new(line: &str) -> Hand { 
        let mut data = line.split_whitespace();
        let hand = data.next().unwrap();
        let bid = data.next().unwrap().parse::<usize>().unwrap(); 
        Hand { 
            bid,
            val: get_val(hand),
            hand_type: get_type(hand),
        }
    }
}

impl PartialOrd for Hand { 
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { 
        match self.hand_type.partial_cmp(&other.hand_type) { 
            Some(Ordering::Equal) => self.val.partial_cmp(&other.val),
            r => r
        }
    }
}

impl Ord for Hand { 
    fn cmp(&self, other: &Self) -> Ordering { 
        match self.hand_type.cmp(&other.hand_type) { 
            Ordering::Equal => self.val.cmp(&other.val),
            r => r
        }
    }
}

#[cfg(test)]
mod test { 
    use std::path::PathBuf;
    use crate::*;

    #[test]
    fn part1_test() { 
        let path = PathBuf::from("example.txt");
        let prod = part1(&path);
        assert_eq!(6440, prod);
    }
}
