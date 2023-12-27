use std::{cmp::Ordering, fs::read_to_string, path::PathBuf};

pub fn part2(path: &PathBuf) -> usize { 
    let reader = read_to_string(path).unwrap();
    let mut hands: Vec<Hand> = reader
        .lines()
        .map(Hand::new)
        .collect();

    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| {
            // println!("{} {:?}", hand._hand, hand.hand_type);
            (i+1) * hand.bid
        })
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
                if i == 11 {
                    continue;
                }
                match (vals[i], vals[11], &hand_type) { 
                    (a, b, _) if a+b==5                                             => hand_type = Type::FiveOfAKind(i),
                    (a, b, t) if t.rank() < 6 && a+b==4                             => hand_type = Type::FourOfAKind(i),
                    (3, 0, Type::OnePair(j)) if vals[*j]==2                         => hand_type = Type::FullHouse(i, j.clone()),
                    (2, 1, Type::OnePair(j)) if vals[*j]==2                         => hand_type = Type::FullHouse(i, j.clone()),
                    (2, 0, Type::ThreeOfAKind(j)) if vals[*j]==3                    => hand_type = Type::FullHouse(i, j.clone()),
                    (2, 1, Type::ThreeOfAKind(j)) if vals[*j]==2                    => hand_type = Type::FullHouse(i, j.clone()),
                    (a, b, t) if t.rank() < 4 && a+b==3                             => hand_type = Type::ThreeOfAKind(i),
                    (a, b, Type::OnePair(j)) if a+b==2 && vals[*j]==2               => hand_type = Type::TwoPair(i, j.clone()),
                    (2, 1, Type::OnePair(j)) if vals[*j]==1                         => hand_type = Type::TwoPair(i, j.clone()),
                    (a, b, t) if t.rank() < 2 && a+b==2                             => hand_type = Type::OnePair(i),
                    (1, 0, Type::NoValue)                                           => hand_type = Type::HighCard(i),
                    _                                                               => continue,
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
            b'J' => 0,  
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
    _hand: String,
}

impl Hand { 
    fn new(line: &str) -> Hand { 
        let mut data = line.split_whitespace();
        let hand = data.next().unwrap();
        let bid = data.next().unwrap().parse::<usize>().unwrap(); 
        Hand { 
            _hand: hand.into(),
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
    use crate::part2::part2;

    #[test]
    fn part2_test() { 
        let path = PathBuf::from("example.txt");
        let prod = part2(&path);
        assert_eq!(5905, prod);
    }
}
