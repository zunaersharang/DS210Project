s_totaluse petgraph::graph::{Graph, NodeIndex};
use petgraph::Undirected;
use std::collections::HashMap;
use std::error::Error;
use csv::ReaderBuilder;

#[derive(Debug, Clone, PartialEq)]
struct Person {
    peer_influence: i32,
    age_group: String,
    socioeconomic_status: String,
    smoking_prevalence: f32,
    drug_experimentation: f32,
}

fn main() -> Result<(), Box<dyn Error>> {

    let data = load_dataset("youth_smoking_drug_data_10000_rows_expanded.csv")?;

    let (graph, node_map) = create_graph(&data);

    let degree_distribution = compute_degree_distribution(&graph);

    let distance_2_neighbors = compute_distance_2_neighbors(&graph);

    let behavior_by_degree = analyze_behavior_by_degree(&graph, &data, &node_map);

    println!("Summary Report:\n");

    println!("Degree Distribution (Top 10 Degrees):");
    for (degree, count) in degree_distribution.iter().take(10) {
        println!("Degree {}: {}", degree, count);
    }

    println!("\nTop 10 Nodes with the Most Distance-2 Neighbors:");
    let mut distance_2_sorted: Vec<_> = distance_2_neighbors.iter().collect();
    distance_2_sorted.sort_by(|a, b| b.1.cmp(a.1)); 
    for (node, count) in distance_2_sorted.iter().take(10) {
        println!("Node {}: {} distance-2 neighbors", node.index(), count);
    }

    println!("\nBehavioral Analysis by Degree (Top 10 Degrees by Avg Smoking Prevalence):");
    let mut behavior_sorted: Vec<_> = behavior_by_degree.iter().collect();
    behavior_sorted.sort_by(|a, b| b.1.0.partial_cmp(&a.1.0).unwrap()); // Sort by smoking prevalence
    for (degree, (smoking_avg, drug_avg)) in behavior_sorted.iter().take(10) {
        println!(
            "Degree {}: Avg Smoking Prevalence = {:.2}, Avg Drug Experimentation = {:.2}",
            degree, smoking_avg, drug_avg
        );
    }

    Ok(())
}


fn load_dataset(path: &str) -> Result<Vec<Person>, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new().has_headers(true).from_path(path)?;
    let mut data = Vec::new();

    for result in reader.records() {
        let record = result?;
        let person = Person {
            peer_influence: record[6].parse::<i32>().unwrap_or(0),
            age_group: record[1].to_string(),
            socioeconomic_status: record[5].to_string(),
            smoking_prevalence: record[3].parse::<f32>().unwrap_or(0.0),
            drug_experimentation: record[4].parse::<f32>().unwrap_or(0.0),
        };
        data.push(person);
    }

    Ok(data)
}


fn create_graph(data: &[Person]) -> (Graph<(), (), Undirected>, HashMap<usize, NodeIndex>) {
    let mut graph = Graph::<(), (), Undirected>::new_undirected();
    let mut node_map = HashMap::new();

    for (index, _) in data.iter().enumerate() {
        let node = graph.add_node(());
        node_map.insert(index, node);
    }

    for (i, person1) in data.iter().enumerate() {
        for (j, person2) in data.iter().enumerate() {
            if i < j && should_connect(person1, person2) {
                graph.add_edge(node_map[&i], node_map[&j], ());
            }
        }
    }

    (graph, node_map)
}


fn should_connect(person1: &Person, person2: &Person) -> bool {
    (person1.peer_influence - person2.peer_influence).abs() <= 2
        && person1.age_group == person2.age_group
        && person1.socioeconomic_status == person2.socioeconomic_status
}


fn compute_degree_distribution(graph: &Graph<(), (), Undirected>) -> HashMap<usize, usize> {
    let mut degree_counts = HashMap::new();

    for node in graph.node_indices() {
        let degree = graph.edges(node).count();
        *degree_counts.entry(degree).or_insert(0) += 1;
    }

    degree_counts
}

fn compute_distance_2_neighbors(graph: &Graph<(), (), Undirected>) -> HashMap<NodeIndex, usize> {
    let mut distance_2_neighbors = HashMap::new();

    for node in graph.node_indices() {
        let mut neighbors_at_distance_2 = std::collections::HashSet::new();

        for neighbor in graph.neighbors(node) {
            for neighbor_of_neighbor in graph.neighbors(neighbor) {
                if neighbor_of_neighbor != node {
                    neighbors_at_distance_2.insert(neighbor_of_neighbor);
                }
            }
        }

        distance_2_neighbors.insert(node, neighbors_at_distance_2.len());
    }

    distance_2_neighbors
}

fn analyze_behavior_by_degree(
    graph: &Graph<(), (), Undirected>,
    data: &[Person],
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

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_connect() {
        let person1 = Person {
            peer_influence: 7,
            age_group: "10-14".to_string(),
            socioeconomic_status: "Low".to_string(),
            smoking_prevalence: 25.0,
            drug_experimentation: 30.0,
        };
        let person2 = Person {
            peer_influence: 8,
            age_group: "10-14".to_string(),
            socioeconomic_status: "Low".to_string(),
            smoking_prevalence: 30.0,
            drug_experimentation: 35.0,
        };
        assert!(should_connect(&person1, &person2));
    }

    #[test]
    fn test_degree_distribution() {
        let data = vec![
            Person {
                peer_influence: 7,
                age_group: "10-14".to_string(),
                socioeconomic_status: "Low".to_string(),
                smoking_prevalence: 25.0,
                drug_experimentation: 30.0,
            },
            Person {
                peer_influence: 8,
                age_group: "10-14".to_string(),
                socioeconomic_status: "Low".to_string(),
                smoking_prevalence: 30.0,
                drug_experimentation: 35.0,
            },
        ];
        let (graph, _) = create_graph(&data);
        let degree_distribution = compute_degree_distribution(&graph);

        assert_eq!(degree_distribution.get(&1), Some(&2)); // Two nodes with degree 1
    }

    #[test]
    fn test_behavior_by_degree() {
        let data = vec![
            Person {
                peer_influence: 7,
                age_group: "10-14".to_string(),
                socioeconomic_status: "Low".to_string(),
                smoking_prevalence: 25.0,
                drug_experimentation: 30.0,
            },
            Person {
                peer_influence: 7,
                age_group: "10-14".to_string(),
                socioeconomic_status: "Low".to_string(),
                smoking_prevalence: 30.0,
                drug_experimentation: 35.0,
            },
        ];
        let (graph, node_map) = create_graph(&data);
        let behavior_by_degree = analyze_behavior_by_degree(&graph, &data, &node_map);

        assert_eq!(
            behavior_by_degree.get(&1),
            Some(&(27.5, 32.5)) // Corrected to match actual output
        );
    }
}



