use std::{fs::read_to_string, str::from_utf8};

fn main() {
    part1();
    part2()
}

fn part1() {
    let mut res: i32 = 0;
    for line in read_to_string("input.txt").unwrap().lines() {
        let bytes = line.as_bytes();
        let first = get_first_ascii_digit(bytes).unwrap().clone();
        let second = get_last_ascii_digit(bytes).unwrap().clone();
        let calibration = from_utf8(&[first, second]).unwrap().parse::<i32>().unwrap();
        res += calibration;
    }
    println!("{}", res)
}

fn get_first_ascii_digit(bytes: &[u8]) -> Option<&u8> {
    match bytes {
        [] => panic!("Digit not found!"),
        [byte, ..] if (byte >= &b'0' && byte <= &b'9') => Some(byte),
        [_, tail @ ..] => get_first_ascii_digit(tail), 
    }
}

fn get_last_ascii_digit(bytes: &[u8]) -> Option<&u8> {
    match bytes {
        [] => None,
        [byte] if (byte >= &b'0' && byte <= &b'9') => Some(byte),
        [byte, tail @ ..] if (byte >= &b'0' && byte <= &b'9') => 
            get_last_ascii_digit(tail).or(Some(byte)), 
        [_, tail @ ..] => get_last_ascii_digit(tail), 
    }
}

fn part2() {
    let mut res: i32 = 0;
    for line in read_to_string("input.txt").unwrap().lines() {
        let mut str = line.to_string();
        let bytes: &mut [u8] = unsafe { str.as_bytes_mut() } ;
        let first = get_first_digit(bytes).unwrap().clone();
        bytes.reverse();
        let second = get_last_digit(bytes).unwrap().clone();
        let calibration = from_utf8(&[first, second]).unwrap().parse::<i32>().unwrap();
        res += calibration;
    }
    println!("{}", res)
}

fn get_first_digit(bytes: &[u8]) -> Option<&u8> {
    match bytes {
        [] => panic!("Digit not found!"),
        [byte, ..] if (byte >= &b'0' && byte <= &b'9') => Some(byte),
        [b'o', b'n', b'e', ..] => Some(&b'1'),
        [b't', b'w', b'o', ..] => Some(&b'2'),
        [b't', b'h', b'r', b'e', b'e', ..] => Some(&b'3'),
        [b'f', b'o', b'u', b'r', ..] => Some(&b'4'),
        [b'f', b'i', b'v', b'e', ..] => Some(&b'5'),
        [b's', b'i', b'x', ..] => Some(&b'6'),
        [b's', b'e', b'v', b'e', b'n', ..] => Some(&b'7'),
        [b'e', b'i', b'g', b'h', b't', ..] => Some(&b'8'),
        [b'n', b'i', b'n', b'e', ..] => Some(&b'9'),
        [_, tail @ ..] => get_first_digit(tail), 
    }
}

fn get_last_digit(bytes: &[u8]) -> Option<&u8> {
    match bytes {
        [] => panic!("Digit not found!"),
        [byte, ..] if (byte >= &b'0' && byte <= &b'9') => Some(byte),
        [b'e', b'n', b'o', ..] => Some(&b'1'),
        [b'o', b'w', b't', ..] => Some(&b'2'),
        [b'e', b'e', b'r', b'h', b't', ..] => Some(&b'3'),
        [b'r', b'u', b'o', b'f', ..] => Some(&b'4'),
        [b'e', b'v', b'i', b'f', ..] => Some(&b'5'),
        [b'x', b'i', b's', ..] => Some(&b'6'),
        [b'n', b'e', b'v', b'e', b's', ..] => Some(&b'7'),
        [b't', b'h', b'g', b'i', b'e', ..] => Some(&b'8'),
        [b'e', b'n', b'i', b'n', ..] => Some(&b'9'),
        [_, tail @ ..] => get_last_digit(tail), 
    }
}
