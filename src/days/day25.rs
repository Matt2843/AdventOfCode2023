use ahash::HashMap;
use itertools::Itertools;
use regex::Regex;
use rustworkx_core::{
    connectivity::stoer_wagner_min_cut,
    petgraph::{graph::UnGraph, Graph, Undirected},
};

pub fn solve(input: &str) -> (usize, usize) {
    let wre = Regex::new(r"(\w+)").unwrap();
    let edge_indexes: HashMap<&str, usize> = input
        .trim()
        .lines()
        .flat_map(|l| wre.find_iter(l))
        .map(|m| m.as_str())
        .unique()
        .enumerate()
        .map(|(idx, node)| (node, idx))
        .collect();
    let edges = input
        .trim()
        .lines()
        .flat_map(|l| l.trim().split_once(':'))
        .flat_map(|(k, v)| v.trim().split_ascii_whitespace().map(move |s| (k, s)))
        .map(|(k, v)| (*edge_indexes.get(k).unwrap(), *edge_indexes.get(v).unwrap()));

    let graph: Graph<usize, usize, Undirected, usize> = UnGraph::from_edges(edges);

    let (_, partition) = stoer_wagner_min_cut(&graph, |_| Ok::<i32, ()>(1))
        .unwrap()
        .unwrap();

    (partition.len() * (graph.node_count() - partition.len()), 0)
}
