use rand::prelude::*;

use std::{
    collections::HashSet,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone)]
pub struct Graph {
    nodes: Vec<ComplexNode>,
    edges: Vec<(String, String)>,
}

impl Graph {
    pub fn new(nodes: Vec<ComplexNode>, edges: Vec<(String, String)>) -> Self {
        Self { nodes, edges }
    }
}

#[derive(Debug, Clone)]
pub struct ComplexNode {
    nodes: HashSet<String>,
}

impl ComplexNode {
    pub fn new(node: &str) -> Self {
        Self {
            nodes: HashSet::from([node.into()]),
        }
    }
}

pub struct KargerResult {
    partition_a: HashSet<String>,
    partition_b: HashSet<String>,
    cut: Vec<(String, String)>,
}

pub fn karger(graph: &Graph, rng: &mut ThreadRng) -> KargerResult {
    let mut graph = graph.clone();
    let nodes = &mut graph.nodes;
    loop {
        if nodes.len() == 2 {
            break;
        }
        let idx = rng.gen_range(0..graph.edges.len());
        let edge_to_merge = graph.edges.swap_remove(idx);
        let a_pos = nodes
            .iter()
            .position(|n| n.nodes.contains(&edge_to_merge.0))
            .unwrap();
        let b_pos = nodes
            .iter()
            .position(|n| n.nodes.contains(&edge_to_merge.1))
            .unwrap();
        contraction(nodes, a_pos.min(b_pos), a_pos.max(b_pos));
        graph.edges.retain(|e| {
            let node = &nodes[a_pos.min(b_pos)];
            !(node.nodes.contains(&e.0) && node.nodes.contains(&e.1))
        })
    }

    KargerResult {
        partition_a: nodes.pop().unwrap().nodes,
        partition_b: nodes.pop().unwrap().nodes,
        cut: graph.edges,
    }
}

fn contraction(nodes: &mut Vec<ComplexNode>, dst: usize, src: usize) {
    let from = nodes.swap_remove(src);
    nodes[dst].nodes.extend(from.nodes.into_iter());
}

fn main() {
    let file = std::fs::File::open("input").unwrap();
    let reader = BufReader::new(file);
    let mut created: HashSet<String> = HashSet::new();
    let mut nodes: Vec<ComplexNode> = Vec::new();
    let mut edges: Vec<(String, String)> = Vec::new();
    for line in reader.lines().map(Result::unwrap) {
        let (src, dst) = line.split_once(':').unwrap();
        edges.extend(dst.split_whitespace().map(|dst| (src.into(), dst.into())));
        for n in [src].into_iter().chain(dst.split_whitespace()) {
            if !created.contains(n) {
                nodes.push(ComplexNode::new(n));
                created.insert(n.into());
            }
        }
    }
    let graph = Graph::new(nodes, edges);
    let mut rng = thread_rng();
    let mut r: KargerResult;
    loop {
        r = karger(&graph, &mut rng);
        if r.cut.len() == 3 {
            break;
        }
    }
    let result = r.partition_a.len() * r.partition_b.len();
    println!("{result}");
}
