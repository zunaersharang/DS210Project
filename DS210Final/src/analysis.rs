use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::EdgeRef;
use petgraph::Undirected;
use std::collections::HashMap;

pub fn compute_degree_distribution(graph: &Graph<(), (), Undirected>) -> HashMap<usize, usize> {
    let mut degree_counts = HashMap::new();

    for node in graph.node_indices() {
        let degree = graph.edges(node).count();
        *degree_counts.entry(degree).or_insert(0) += 1;
    }

    degree_counts
}

pub fn analyze_behavior_by_degree(
    graph: &Graph<(), (), Undirected>,
    data: &[crate::graph::Person],
    node_map: &HashMap<usize, NodeIndex>,
) -> HashMap<usize, (f32, f32)> {
    let mut behavior_by_degree = HashMap::new();
    let mut degree_totals = HashMap::new();

    for (index, &node) in node_map.iter() {
        let degree = graph.edges(node).count();
        let smoking = data[*index].smoking_prevalence;
        let drug = data[*index].drug_experimentation;

        behavior_by_degree
            .entry(degree)
            .and_modify(|(s_total, d_total)| {
                *s_total += smoking;
                *d_total += drug;
            })
            .or_insert((smoking, drug));

        degree_totals
            .entry(degree)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    behavior_by_degree.iter_mut().for_each(|(degree, (s_total, d_total))| {
        let count = degree_totals[degree] as f32;
        *s_total /= count;
        *d_total /= count;
    });

    behavior_by_degree
}

pub fn compute_distance_2_neighbors(
    graph: &Graph<(), (), Undirected>,
) -> HashMap<NodeIndex, usize> {
    let mut distance_2_counts = HashMap::new();

    for node in graph.node_indices() {
        let mut neighbors = HashMap::new();

        for edge in graph.edges(node) {
            let neighbor = edge.target();
            for edge_2 in graph.edges(neighbor) {
                let neighbor_2 = edge_2.target();
                if neighbor_2 != node {
                    neighbors.entry(neighbor_2).or_insert(());
                }
            }
        }

        distance_2_counts.insert(node, neighbors.len());
    }

    distance_2_counts
}


