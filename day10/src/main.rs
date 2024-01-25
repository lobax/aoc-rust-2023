use std::{fs::read_to_string, path::PathBuf};

fn main() {
    let input = PathBuf::from("input.txt");
    let res_1 = part1(&input);
    println!("{}", res_1);
    let res_2 = part2(&input);
    println!("{}", res_2);
}

fn part1(path: &PathBuf) -> usize { 
    let reader = read_to_string(path).unwrap();
    let mut maze = Maze::new();
    for line in reader.lines() { 
        maze.add(line.as_bytes());
    }
    let walker = MazeWalker { maze: &mut maze, distance: 0 };
    let res = walker.into_iter().last().unwrap();
    res
}

fn part2(path: &PathBuf) -> usize { 
    let reader = read_to_string(path).unwrap();
    let mut maze = Maze::new();
    for line in reader.lines() { 
        maze.add(line.as_bytes());
    }
    maze.padd();
    let walker = MazeWalker { maze: &mut maze, distance: 0 };
    walker.into_iter().last().unwrap();
    let area_walker = MazeAreaWalker { maze: &mut maze, pos: vec!((0, 0)), count: 0 };
    area_walker.into_iter().last().unwrap();
    maze.unpadd();
    maze.get_inside_count()
}

type Coords = (usize, usize);
type Icoords = (isize, isize);

fn convert((r, c): Coords) -> Icoords { 
    (r as isize, c as isize)
}

fn remove_nth(arr: &Vec<Vec<u8>>, n: usize) -> Vec<Vec<u8>> { 
    arr.iter().enumerate()
        .filter(|&(i,_)| i % n != 0)
        .map(|(_,r)| r.iter()
            .enumerate()
            .filter(|&(i,_)| i % n != 0)
            .map(|(_,v)| v.clone())
            .collect())
        .collect()
}

struct Maze { 
    _maze: Vec<Vec<u8>>,
    _visited_pipes: Vec<Vec<u8>>,
    _visited_area: Vec<Vec<u8>>,
    pos: Vec<Coords>,
}

impl Maze { 
    fn new() -> Self { 
        Self { 
            _maze: Vec::new(),
            _visited_pipes: Vec::new(),
            _visited_area: Vec::new(),
            pos: Vec::new(),
        }
    }

    fn padd(&mut self) {
        let mut padded: Vec<Vec<u8>> = vec!();
        for (r, row) in self._maze.iter().enumerate() { 
            let mut above = vec!();
            let mut cur = vec!();
            for (c, val) in row.iter().enumerate() {
                match val { 
                    b'-' => {
                        above   .append(&mut vec!(b'.', b'.'));
                        cur     .append(&mut vec!(b'-', b'-'));
                    }
                    b'|' => {
                        above   .append(&mut vec!(b'.', b'|'));
                        cur     .append(&mut vec!(b'.', b'|'));
                    }
                    b'7' => {
                        above   .append(&mut vec!(b'.', b'.'));
                        cur     .append(&mut vec!(b'-', b'7'));
                    }
                    b'F' => {
                        above   .append(&mut vec!(b'.', b'.'));
                        cur     .append(&mut vec!(b'.', b'F'));
                    }
                    b'J' => {
                        above   .append(&mut vec!(b'.', b'|'));
                        cur     .append(&mut vec!(b'-', b'J'));
                    }
                    b'L' => {
                        above   .append(&mut vec!(b'.', b'|'));
                        cur     .append(&mut vec!(b'.', b'L'));
                    }
                    b'S' => {
                        let start: Vec<_> = self.parse_start((r,c));

                        if r > 0 && start.contains(&(r-1,c)) { 
                            above.append(&mut vec!(b'.', b'|'));
                        } else { 
                            above.append(&mut vec!(b'.', b'.'));
                        }

                        if c > 0 && start.contains(&(r,c-1)) { 
                            cur.append(&mut vec!(b'-', b'S'));
                        } else { 
                            cur.append(&mut vec!(b'.', b'S'));
                        }

                    }
                    _ => { 
                        above   .append(&mut vec!(b'.', b'.'));
                        cur     .append(&mut vec!(b'.', b'.'));
                    }
                }
            }
            above.push(b'.');
            cur.push(b'.');
            padded.push(above);
            padded.push(cur);
        }
        padded.push(vec![b'.'; self._maze[0].len()*2 + 1]);
        self._visited_pipes = padded.clone();
        self._visited_area = padded.clone();
        self._maze = padded;
        self.pos = self.pos.iter().map(|(r,c)| (r*2+1, c*2+1)).collect();
    }

    fn unpadd(&mut self) { 
        self._maze = remove_nth(&self._maze, 2);
        self._visited_pipes= remove_nth(&self._visited_pipes, 2);
        self._visited_area= remove_nth(&self._visited_area, 2);
    }

    fn add<'a>(&mut self, row: &'a [u8]) { 
        if let Some(col) = row.iter().position(|&v| v == b'S') { 
            let row = self._maze.len();
            self.pos.push((row, col));
        }
        self._maze.push(row.to_vec());
        self._visited_pipes.push(row.to_vec());
        self._visited_area.push(row.to_vec());
    }

    fn get(&self, coord: Icoords) -> Option<u8> {
        match coord { 
            (r, _) if r < 0                                         => None,
            (r, _) if r as usize >= self._maze.len()                => None,
            (_, c) if c < 0                                         => None,
            (r, c) if c as usize >= self._maze[r as usize].len()    => None,
            (r, c)                                                  => Some(self._maze[r as usize][c as usize])
        } 
    }

    fn exists(&self, coord: Icoords) -> Option<Coords> {
        match coord { 
            (r, _) if r < 0                                         => None,
            (r, _) if r as usize >= self._maze.len()                => None,
            (_, c) if c < 0                                         => None,
            (r, c) if c as usize >= self._maze[r as usize].len()    => None,
            (r, c)                                                  => Some((r as usize, c as usize))
        }
    }

    fn parse_start(&self, (r, c): Coords) -> Vec<Coords> { 
        let (ir, ic) = convert((r, c));
        let up      = self.get((ir-1, ic));
        let down    = self.get((ir+1, ic));
        let left    = self.get((ir, ic-1));
        let right   = self.get((ir, ic+1));

        let mut res = Vec::new();

        if let Some(b'|' | b'7' | b'F') = up { 
            res.push((r-1, c));
        }

        if let Some(b'|' | b'J' | b'L') = down { 
            res.push((r+1, c));
        }

        if let Some(b'-' | b'F' | b'L') = left { 
            res.push((r, c-1));
        }

        if let Some(b'-' | b'J' | b'7') = right { 
            res.push((r, c+1));
        }

        if res.len() != 2 { 
            panic!("Unable to parse starting point!")
        }
        res
    }

    fn step_pipe(&self, (r, c): Coords) -> Vec<Coords> { 
        let (ir, ic) = convert((r, c));
        let up      = self.exists((ir-1, ic));
        let down    = self.exists((ir+1, ic));
        let left    = self.exists((ir, ic-1));
        let right   = self.exists((ir, ic+1));

        match self._maze[r][c] { 
            b'|'    => vec!(up, down),
            b'-'    => vec!(left, right),
            b'L'    => vec!(up, right),
            b'J'    => vec!(up, left),
            b'7'    => vec!(down, left),
            b'F'    => vec!(down, right),
            b'S'    => self.parse_start((r,c)).iter().map(|&c| Some(c)).collect(),
            b'.'    => vec!(),
            _       => vec!(),
        }.iter().filter_map(|&c| c).collect()
    }

    fn mark_pipe(&mut self, (r, c): Coords) { 
        self._visited_pipes[r][c] = b'#';
    }

    fn step_outside(&self, (r, c): Coords) -> Vec<Coords> { 
        let (ir, ic) = convert((r, c));
        let up      = self.exists((ir-1, ic));
        let down    = self.exists((ir+1, ic));
        let left    = self.exists((ir, ic-1));
        let right   = self.exists((ir, ic+1));

        match (self._maze[r][c], self._visited_pipes[r][c]) { 
            (b'|', b'#')    => vec!(up, down),
            (b'-', b'#')    => vec!(left, right),
            (b'L', b'#')    => vec!(up, right),
            (b'J', b'#')    => vec!(up, left),
            (b'7', b'#')    => vec!(down, left),
            (b'F', b'#')    => vec!(down, right),
            (b'S', b'#')    => self.parse_start((r,c)).iter().map(|&c| Some(c)).collect(),
            (_,_)           => vec!(up,down,left,right),
        }.iter().filter_map(|&c| c).collect()
    }

    fn mark_outside(&mut self, (r, c): Coords) { 
        match self._visited_pipes[r][c] {
            b'#'    => self._visited_area[r][c] = b'#',
            _       => self._visited_area[r][c] = b'O',
        }
    }

    fn unmarked_pipes(&self, (r,c): Coords) -> Option<Coords> { 
        match self._visited_pipes[r][c] { 
            b'#'  => None,
            _     => Some((r,c)),
        }
    }

    fn unmarked_outside(&self, (r,c): Coords) -> Option<Coords> { 
        match self._visited_area[r][c] { 
            b'#'  => None,
            b'O'  => None,
            _     => Some((r,c)),
        }
    }

    fn get_inside_count(&self) -> usize { 
        let mut res = 0;
        for row in self._visited_area.iter() {
            for &byte in row.iter() {
                if byte != b'#' && byte != b'O' {
                    res += 1;
                }
            }
        }
        res
    }

    fn print(&self) {
        println!("Maze:");
        for row in self._maze.iter() {
            println!("{}", String::from_utf8_lossy(row));
        }
        println!("Visited area:");
        for row in self._visited_area.iter() {
            println!("{}", String::from_utf8_lossy(row));
        }
        println!();
    }
}

struct MazeWalker<'a> { 
    maze: &'a mut Maze,
    distance: usize,
}

impl<'a> Iterator for MazeWalker<'a> { 
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> { 
        let next_steps = self.maze.pos.iter()
            .flat_map(|&coord| self.maze.step_pipe(coord))
            .filter_map(|coord| self.maze.unmarked_pipes(coord))
            .collect();

        for i in 0..self.maze.pos.len() { 
            self.maze.mark_pipe(self.maze.pos[i]);
        }

        self.maze.pos = next_steps;

        match self.maze.pos.len() {
            0 => None,
            1 => None,
            _ => { 
                self.distance += 1;
                Some(self.distance)
            }
        }
    }
}

struct MazeAreaWalker<'a> { 
    maze: &'a mut Maze,
    pos: Vec<Coords>,
    count: usize,
}

impl<'a> Iterator for MazeAreaWalker<'a> { 
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> { 

        let mut unmarked_next_steps: Vec<Coords> = self.pos.iter()
            .flat_map(|&coord| self.maze.step_outside(coord))
            .filter_map(|coord| self.maze.unmarked_outside(coord))
            .collect();

        unmarked_next_steps.sort();
        unmarked_next_steps.dedup();

        for i in 0..self.pos.len() { 
            self.maze.mark_outside(self.pos[i]);
        }

        self.pos = unmarked_next_steps;

        match self.pos.len() {
            0 => None,
            _ => { 
                self.count += 1;
                Some(self.count)
            }
        }
    }
}


#[cfg(test)]
mod test { 
    use std::path::PathBuf;
    use crate::*;

    #[test]
    fn problem1_example() {
        let path = PathBuf::from("example.txt");
        let steps = part1(&path);
        assert_eq!(4, steps);
    }

    #[test]
    fn problem1_example2() {
        let path = PathBuf::from("example2.txt");
        let steps = part1(&path);
        assert_eq!(4, steps);
    }

    #[test]
    fn problem1_example3() {
        let path = PathBuf::from("example3.txt");
        let steps = part1(&path);
        assert_eq!(8, steps);
    }

    #[test]
    fn problem1_example4() {
        let path = PathBuf::from("example4.txt");
        let steps = part1(&path);
        assert_eq!(8, steps);
    }

    #[test]
    fn problem2_example5() {
        let path = PathBuf::from("example5.txt");
        let steps = part2(&path);
        assert_eq!(4, steps);
    }

    #[test]
    fn problem2_example6() {
        let path = PathBuf::from("example6.txt");
        let steps = part2(&path);
        assert_eq!(8, steps);
    }

    #[test]
    fn problem2_example7() {
        let path = PathBuf::from("example7.txt");
        let steps = part2(&path);
        assert_eq!(10, steps);
    }

}
