use std::{collections::HashMap, fs::read_to_string, path::PathBuf};

fn main() {
    let input = PathBuf::from("input.txt");
    let res_1 = part1(&input);
    println!("{}", res_1);
    let res_2 = part2(&input);
    println!("{}", res_2);
}

fn part1(path: &PathBuf) -> usize { 
    let reader = read_to_string(path).unwrap();
    let mut lines = reader.lines();
    let instructions = lines.next().unwrap().as_bytes();

    let _ = lines.next();

    let mut graph = Graph::new();

    for line in lines { 
        let node: String  = line[0..3].to_string().clone();
        let left: String = line[7..10].to_string().clone();
        let right: String = line[12..15].to_string().clone();

        graph.add(node, NodeNeighbors{left, right})
    }

    graph.navigate("AAA".into(), "ZZZ".into(), instructions)
}

/// Brute force because LCM is cheating
fn part2 (path: &PathBuf) -> InstructionPtr { 
    let reader = read_to_string(path).unwrap();
    let mut lines = reader.lines();
    let instructions = lines.next().unwrap().as_bytes();

    let _ = lines.next();

    let mut graph = Graph::new();
    let mut start_nodes = Vec::new();

    for line in lines { 
        let node: String  = line[0..3].to_string().clone();
        let left: String = line[7..10].to_string().clone();
        let right: String = line[12..15].to_string().clone();

        graph.add(node.clone(), NodeNeighbors{left, right});
        if node.ends_with("A") { 
            start_nodes.push(node.clone());
        }
    }

    println!{"Start nodes: {:?}", start_nodes};

    let mut walkers: Vec<_> = start_nodes
        .into_iter()
        .map(|node| GhostWalker::new(&graph, node, &instructions))
        .map(|w| w.into_iter())
        .collect();
    let mut idxs: Vec<InstructionPtr> = vec![0; walkers.len()];
    let mut max: InstructionPtr = 1;
    loop { 
        for i in 0..walkers.len() { 
            while idxs[i] < max {
                idxs[i] = walkers[i].next().unwrap();
                if idxs[i] >= max {
                    max = idxs[i];
                }
            }
        }
        if idxs.iter().all(|i| i == &max) {
            return max;
        }
    }
}

type Node = String;

#[derive(Debug)]
struct NodeNeighbors { 
    left: Node,
    right: Node,
}

#[derive(Debug)]
struct Graph { 
    _graph: HashMap<Node, NodeNeighbors>,
}

impl Graph { 
    fn new() -> Graph { 
        Graph {
            _graph: HashMap::new(),
        }
    }

    fn add(&mut self, node: Node, neighbors: NodeNeighbors) { 
        self._graph.insert(node, neighbors);
    }

    fn navigate(&self, start: Node, target: Node, instructions: &[u8]) -> usize { 
        let mut steps = 0;
        let mut node = &start;

        loop {
            for instruction in instructions {
                node = self.step(node, instruction);
                steps += 1;
                if node == &target { 
                    return steps
                }
            }
        }
    }

    fn step(&self, pos: &Node, instruction: &u8) -> &Node {
        let neighbors = self._graph.get(pos).expect("Node not found");
        match instruction {
            b'L' => &neighbors.left,
            b'R' => &neighbors.right,
            _ => panic!("Unknown instruction")
        }
    }
}

type InstructionPtr = u64;

#[derive(Debug)]
struct Jump { 
    idx_jmp: InstructionPtr,
    node: Node,
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct GraphState { 
    node: Node,
    idx: usize, // Needs to be modulo the instruction buffer size
}

#[derive(Debug)]
struct GhostWalker<'a> { 
    graph: &'a Graph,
    ins_buffer: &'a [u8],
    _state: Node,
    _cache: HashMap<GraphState, Jump>, 
    _idx: InstructionPtr,
}

impl<'a> GhostWalker<'a> { 
    fn new(graph: &'a Graph, start: Node, instructions: &'a [u8]) -> Self { 
        GhostWalker { 
            graph,
            _state: start,
            ins_buffer: instructions,
            _cache: HashMap::new(),
            _idx: 0
        }
    }
}

impl<'a> Iterator for GhostWalker<'a> { 
    type Item = InstructionPtr;

    fn next(&mut self) -> Option<Self::Item> { 
        let mut idx_jmp: InstructionPtr = 0;
        let start_node = self._state.clone();
        let idx = (self._idx % (self.ins_buffer.len() as InstructionPtr)) as usize;
        let graph_state = GraphState { node: start_node, idx };
        loop {
            // Check the cache to see if we can fast forward
            if let Some(jump) = self._cache.get(&graph_state) { 
                self._idx += jump.idx_jmp;
                self._state = jump.node.clone();
                return Some(self._idx);
            }

            let idx = (self._idx % (self.ins_buffer.len() as InstructionPtr)) as usize;
            let ins = self.ins_buffer[idx].clone();
            let next_node = self.graph.step(&self._state, &ins);

            self._idx += 1;
            idx_jmp += 1;
            self._state = next_node.clone();

            if next_node.ends_with("Z") {
                self._cache.insert(graph_state, Jump{idx_jmp, node: next_node.clone()});
                return Some(self._idx); 
            }
        }
    }
}
#[cfg(test)]
mod test { 
    use std::path::PathBuf;
    use crate::*;

    #[test]
    fn problem1() {
        let path = PathBuf::from("example.txt");
        let steps = part1(&path);
        assert_eq!(2, steps);

        let path = PathBuf::from("example2.txt");
        let steps = part1(&path);
        assert_eq!(6, steps);
    }

    #[test]
    fn problem2() {
        let path = PathBuf::from("example3.txt");
        let steps = part2(&path);
        assert_eq!(6, steps);

    }

}
