use std::{
    collections::HashSet,
    fs::read_to_string, 
    path::PathBuf};

fn main() {
    let input = PathBuf::from("input.txt");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn parse_numbers<'a>(numbers: impl IntoIterator<Item=&'a str>) -> HashSet<i32> { 
    numbers
        .into_iter()
        .filter_map(|n| n.parse::<i32>().ok())
        .collect()
}

fn part1(input: &PathBuf) -> usize {
    let mut res: usize = 0;
    for line in read_to_string(input).unwrap().lines() {
        let data = line.split(": ").last().unwrap();
        let mut card = data.split(" | ");
        let winning_numbers= parse_numbers(card.next().unwrap().split_whitespace());
        let my_numbers = parse_numbers(card.next().unwrap().split_whitespace());
        let my_winning_numbers = winning_numbers.intersection(&my_numbers); 
        let base: usize = 2;
        let val = match my_winning_numbers.count() as u32 { 
            0 => 0,
            p => base.pow(p-1)
        };
        res += val;
    }
    res
}

struct Card { 
    copies: usize,
    winning_numbers: usize
}

impl Card {
    fn add_copies(&mut self, n: usize) { 
        self.copies += n;
    }
}

fn get_card_idx(card_str: &str) -> usize { 
    let idx_str = card_str.split_whitespace().last().unwrap();
    idx_str.parse::<usize>().unwrap()
}

fn part2(input: &PathBuf) -> usize {
    let mut cards: Vec<Card> = Vec::new();
    for line in read_to_string(input).unwrap().lines() {
        let mut data = line.split(": ");
        let mut card = data.last().unwrap().split(" | ");
        let winning_numbers= parse_numbers(card.next().unwrap().split_whitespace());
        let card_numbers = parse_numbers(card.next().unwrap().split_whitespace());
        let card_winning_numbers = winning_numbers.intersection(&card_numbers); 
        cards.push(Card {
            copies: 1, 
            winning_numbers: card_winning_numbers.count()
        });
    }

    for idx in 1..cards.len() { 
        let card = &cards[idx - 1];
        let copies = card.copies.clone();
        // println!("copies: {} winning numbers: {}", copies, card.winning_numbers);
        for i in idx..(idx + card.winning_numbers)
        { 
            if i < cards.len() { 
                // println!("Adding {} copies to {}", copies, i);
                let _card = &mut cards[i];
                _card.add_copies(copies);
            }
        }
    }
    let res: usize = cards.iter().map(|c| c.copies ).sum();
    // println!("{}", res);
    res
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;
    use crate::{part1, part2};

    #[test]
    fn problem_1() {
        let path = PathBuf::from("example.txt");
        let sum = part1(&path);
        assert_eq!(13, sum)
    }
    #[test]
    fn problem_2() {
        let path = PathBuf::from("example.txt");
        let sum = part2(&path);
        assert_eq!(30, sum)
    }
}
