use csv::ReaderBuilder;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::Undirected;
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Clone, PartialEq)]
pub struct Person {
    pub peer_influence: i32,
    pub age_group: String,
    pub socioeconomic_status: String,
    pub smoking_prevalence: f32,
    pub drug_experimentation: f32,
}

pub fn load_dataset(path: &str) -> Result<Vec<Person>, Box<dyn Error>> {
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

pub fn create_graph(data: &[Person]) -> (Graph<(), (), Undirected>, HashMap<usize, NodeIndex>) {
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

pub fn should_connect(person1: &Person, person2: &Person) -> bool {
    (person1.peer_influence - person2.peer_influence).abs() <= 2
        && person1.age_group == person2.age_group
        && person1.socioeconomic_status == person2.socioeconomic_status
}