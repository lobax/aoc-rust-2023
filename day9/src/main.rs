use std::{fs::read_to_string, path::PathBuf};

fn main() {
    let input = PathBuf::from("input.txt");
    let res_1 = part1(&input);
    println!("{}", res_1);
    let res_2 = part2(&input);
    println!("{}", res_2);
}

fn part1(path: &PathBuf) -> i32 { 
    let reader = read_to_string(path).unwrap();
    reader.lines()
        .map(|l| History::new(l))
        .map(|h| h.predict())
        .sum()
}

fn part2(path: &PathBuf) -> i32 { 
    let reader = read_to_string(path).unwrap();
    reader.lines()
        .map(|l| History::new(l))
        .map(|h| h.reverse())
        .map(|h| h.predict())
        .sum()
}

struct History { 
    history: Vec<i32>,
}

impl History { 
    fn new(_history: &str) -> Self { 
        let history = _history
            .split_whitespace()
            .filter_map(|v| v.parse::<i32>().ok())
            .collect();
        Self { history }
    }

    fn reverse(&self) -> Self { 
        let mut history = self.history.clone();
        history.reverse();
        Self { 
            history 
        }
    }

    fn predict(&self) -> i32 { 
        let mut diff_vectors = Vec::new();
        diff_vectors.push(self.history.clone());
        let mut diff_v = diff_vector(&self.history);

        while diff_v.iter().any(|v| v != &0) { 
            diff_vectors.push(diff_v.clone());
            diff_v = diff_vector(&diff_v);
        }

        let mut diff = 0;
        for v_i in 1..diff_vectors.len() { 
            let last = diff_vectors[v_i].pop().unwrap();
            diff += last;
        }
        self.history[self.history.len() -1] + diff
    }
}

fn diff_vector(vector: &Vec<i32>) -> Vec<i32> { 
    let mut diff_vector = Vec::new();
    for i in 1..vector.len() { 
        diff_vector.push(vector[i] - vector[i-1]);
    }
    diff_vector
}


#[cfg(test)]
mod test { 
    use std::path::PathBuf;
    use crate::*;

    #[test]
    fn problem1() {
        let path = PathBuf::from("example.txt");
        let steps = part1(&path);
        assert_eq!(114, steps);
    }

    #[test]
    fn problem2() {
        let path = PathBuf::from("example.txt");
        let steps = part2(&path);
        assert_eq!(2, steps);
    }
}
