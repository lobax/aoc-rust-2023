use std::{collections::{VecDeque, HashSet}, hash::{Hash, Hasher}, fs::read_to_string, path::PathBuf, str::from_utf8};
use std::collections::hash_map::DefaultHasher;

#[derive(Default, Debug, PartialEq, Eq, Hash, Clone)]
struct Part {
    col: usize,
    buf_hash: u64,
    val: i32,
}

#[derive(Debug)]
struct Symbol {
    pos: usize, 
    val: u8
}

#[derive(Hash)]
struct Buffer <'a> { 
    buffer: &'a [u8], 
}

struct Symbols <'a> { 
    buf: &'a Buffer<'a>, 
    pos: usize,
}

impl <'a> Iterator for Symbols <'a> {
    type Item = Symbol;

    fn next(&mut self) -> Option<Self::Item> { 
        let pos = self.pos.clone();
        self.pos += 1;
        match &self.buf.buffer[(pos as usize)..] {
            [] => {
                self.pos = 0;
                None
            }
            [b'.', ..] => self.next(),
            [byte, ..] if !is_number(byte) => Some(Symbol {pos, val: byte.clone()}),
            _ => self.next(),
        }
    }
}

struct Gears<'a> { 
    buf: &'a Buffer<'a>, 
    pos: usize,
}

impl <'a> Iterator for Gears<'a> {
    type Item = Symbol;

    fn next(&mut self) -> Option<Self::Item> { 
        let pos = self.pos.clone();
        self.pos += 1;
        match &self.buf.buffer[(pos as usize)..] {
            [] => {
                self.pos = 0;
                None
            }
            [b'*', ..] => Some(Symbol {pos, val: b'*'}),
            _ => self.next(),
        }
    }
}

fn main() {
    let input = PathBuf::from("input.txt");
    let res_1 = part1(&input);
    println!("{}", res_1);
    let res_2 = part2(&input);
    println!("{}", res_2);
}

fn part1(input: &PathBuf) -> i32 {
    let mut buffers: VecDeque<Buffer> = VecDeque::new();
    let mut parts = HashSet::new(); 
    for line in read_to_string(input).unwrap().lines() {
        let bytes = line.as_bytes();
        if buffers.len() == 3 {
            buffers.pop_front();
        }
        buffers.push_back(Buffer { buffer: bytes});
        if buffers.len() == 3 {
            let _buffer = buffers.get(1).unwrap();
            let symbols = Symbols { pos: 0, buf: _buffer}; 
            for symbol in symbols { 
                for buffer in &buffers {
                    let pos = symbol.pos as isize;

                    if let Some(number) = parse_number(pos-1, buffer) {
                        parts.insert(number);
                    }

                    if let Some(number) = parse_number(pos, buffer) {
                        parts.insert(number);
                    }

                    if let Some(number) = parse_number(pos+1, buffer) {
                        parts.insert(number);
                    }
                }
            }
            
        }
    }
    parts.iter().map(|p| p.val).sum()
}

fn part2(input: &PathBuf) -> i32 {
    let mut buffers: VecDeque<Buffer> = VecDeque::new();
    let mut gear_ratio = 0; 
    for line in read_to_string(input).unwrap().lines() {
        let bytes = line.as_bytes();
        if buffers.len() == 3 {
            buffers.pop_front();
        }
        buffers.push_back(Buffer { buffer: bytes});
        if buffers.len() == 3 {
            let _buffer = buffers.get(1).unwrap();
            let gears = Gears { pos: 0, buf: _buffer}; 
            for gear in gears { 
                let mut gear_parts = HashSet::new();
                for buffer in &buffers {
                    let pos = gear.pos as isize;

                    if let Some(number) = parse_number(pos-1, buffer) {
                        gear_parts.insert(number);
                    }

                    if let Some(number) = parse_number(pos, buffer) {
                        gear_parts.insert(number);
                    }

                    if let Some(number) = parse_number(pos+1, buffer) {
                        gear_parts.insert(number);
                    }
                }
                if gear_parts.len() == 2 { 
                    gear_ratio += gear_parts.into_iter().map(|p| p.val).reduce(|a,b| a*b).unwrap();
                }
            }
        }
    }
    gear_ratio
}

fn parse_number(idx: isize, buf: &Buffer) -> Option<Part> {
    if idx < 0 { 
        return None;
    }
    if idx as usize >= buf.buffer.len() { 
        return None;
    }
    let b = buf.buffer.get(idx as usize).unwrap();
    if !is_number(b) { 
        return None;
    }
    let (col, mut number) = get_preceding(idx as usize, buf.buffer);
    number.append(&mut vec!(b.clone()));
    number.append(&mut get_proceeding(idx as usize, buf.buffer));
    let digit = parse_digit(&number[..]);
    let mut hasher = DefaultHasher::new(); 
    buf.hash(&mut hasher);
    Some(Part { 
        col,
        buf_hash: hasher.finish(),
        val: digit,
    })
}

fn get_preceding(idx: usize, buffer: &[u8]) -> (usize, Vec<u8>) {
    if idx == 0 {
        return (0, Vec::new());
    }

    match &buffer[(idx - 1)..] {
        [b, ..] if is_number(b) => {
            let (i, mut res) = get_preceding(idx-1, buffer);
            let mut vec = vec!(b.clone());
            res.append(&mut vec);
            (i, res)
        }
        _ => (idx, Vec::new()),
    }

}

fn get_proceeding(idx: usize, buffer: &[u8]) -> Vec<u8> {
    if buffer.len() - 1 == idx {
        return Vec::new();
    }

    match &buffer[(idx + 1)..] {
        [b, ..] if is_number(b) => {
            let mut vec = get_proceeding(idx+1, buffer);
            let mut res = vec!(b.clone());
            res.append(&mut vec);
            res
        }
        _ => Vec::new(),
    }

}

fn is_number(byte: &u8) -> bool {
    byte >= &b'0' && byte <= &b'9'
}

fn parse_digit(bytes: &[u8]) -> i32 {
    from_utf8(bytes).unwrap().parse::<i32>().unwrap()
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;
    use crate::{part1, part2, parse_number, Symbols, Buffer};

    #[test]
    fn problem_1() {
        let path = PathBuf::from("example.txt");
        let sum = part1(&path);
        assert_eq!(4361, sum)
    }

    #[test]
    fn problem_2() {
        let path = PathBuf::from("example.txt");
        let sum = part2(&path);
        assert_eq!(467835, sum)
    }

    #[test]
    fn test_iter() {
        let _buffer = [b'.', b'.', b'a', b'.', b'.', b'b', b'.'];
        let buffer = Buffer {buffer: &_buffer};
        let symbols = Symbols { pos: 0, buf: &buffer };
        let mut iter = symbols.into_iter(); 
        let mut symbol = iter.next();
        assert_eq!(Some(b'a'), symbol.map(|s| s.val));
        symbol = iter.next();
        assert_eq!(Some(b'b'), symbol.map(|s| s.val));
        symbol = iter.next();
        assert_eq!(None, symbol.map(|s| s.val));

    }

    #[test]
    fn parse_numbers() {
        let mut  buffer = Buffer {buffer: &[b'1', b'0', b'1']};
        let mut number = parse_number(1, &buffer).map(|p| p.val);
        assert_eq!(number, Some(101));

        buffer = Buffer {buffer: &[b'9', b'8', b'7']};
        number = parse_number(0, &buffer).map(|p| p.val);
        assert_eq!(number, Some(987));

        buffer = Buffer {buffer: &[b'.', b'8', b'9']};
        number = parse_number(1, &buffer).map(|p| p.val);
        assert_eq!(number, Some(89));

        buffer = Buffer {buffer: &[b'.', b'8', b'9']};
        number = parse_number(2, &buffer).map(|p| p.val);
        assert_eq!(number, Some(89));

        buffer = Buffer {buffer: &[b'.', b'9', b'.']};
        number = parse_number(1, &buffer).map(|p| p.val);
        assert_eq!(number, Some(9));

        buffer = Buffer {buffer: &[b'.', b'.', b'.']};
        number = parse_number(1, &buffer).map(|p| p.val);
        assert_eq!(number, None);

        buffer = Buffer {buffer: &[b'.', b'a', b'.']};
        number = parse_number(1, &buffer).map(|p| p.val);
        assert_eq!(number, None);

        buffer = Buffer {buffer: &[b'9', b'9', b'9']};
        number = parse_number(-1, &buffer).map(|p| p.val);
        assert_eq!(number, None);

        buffer = Buffer {buffer: &[b'9', b'9', b'9']};
        number = parse_number(3, &buffer).map(|p| p.val);
        assert_eq!(number, None);
    }

}
