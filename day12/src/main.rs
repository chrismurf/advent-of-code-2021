use indexmap::IndexMap;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug)]
struct Node {
    id: u32,
    _name: String,
    revisitable: bool,
}

#[derive(Debug)]
struct Graph {
    // IndexMap will let us look up by string *or* index
    nodes: IndexMap<String, Node>,
    edges: HashMap<u32, HashSet<u32>>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            nodes: IndexMap::new(),
            edges: HashMap::new(),
        }
    }

    fn from_file(path: &str) -> Self {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let mut graph: Graph = Graph::new();
        for line in reader.lines() {
            let names: Vec<String> = line.unwrap().split("-").map(|x| x.to_owned()).collect();
            graph.add_edge(names.get(0).unwrap(), names.get(1).unwrap());
        }
        graph
    }

    fn ensure_node_exists(&mut self, name: &str) -> u32 {
        let entry = self.nodes.entry(name.to_string());
        let new_id: u32 = entry.index() as u32;
        let node: &mut Node = entry.or_insert(Node {
            id: 0,
            _name: name.to_string(),
            revisitable: name.to_ascii_uppercase() == name,
        });
        node.id = new_id;
        node.id
    }

    fn add_edge(&mut self, a: &str, b: &str) {
        let id_a = self.ensure_node_exists(a);
        let id_b = self.ensure_node_exists(b);
        self.edges
            .entry(id_a)
            .or_insert(HashSet::new())
            .insert(id_b);
        self.edges
            .entry(id_b)
            .or_insert(HashSet::new())
            .insert(id_a);
    }

    fn generate_paths_via(
        &self,
        prefix: Vec<u32>,
        destination: u32,
        allow_revisit: bool,
    ) -> Vec<Vec<u32>> {
        let mut paths = Vec::new();

        if let Some(edges) = self.edges.get(prefix.last().unwrap()) {
            for &next_id in edges {
                let mut new_prefix = prefix.clone();
                new_prefix.push(next_id);

                // Option 1 - next_id is 'end', add to paths and continue with next edge
                if next_id == destination {
                    paths.push(new_prefix);
                    continue;
                }
                let (_, next_node) = self.nodes.get_index(next_id as usize).unwrap();
                if next_node.revisitable || !prefix.contains(&next_id) {
                    // Option 2 - next_id is for a revisitable or unvisited node
                    paths.extend(self.generate_paths_via(new_prefix, destination, allow_revisit));
                } else if prefix.contains(&next_id)
                    && allow_revisit
                    && next_id != *prefix.first().unwrap()
                {
                    // Option 3 - visited, non-revisitable node - but we can get away with that once (but not start!)
                    paths.extend(self.generate_paths_via(new_prefix, destination, false));
                }

                // Option 4 - visited, non-revisitable node - ignore that path
            }
        }
        paths
    }

    fn count_paths(&self, allow_revisit: bool) -> u32 {
        let initial_path: Vec<u32> = vec![self.nodes.get("start").unwrap().id];
        let paths = self.generate_paths_via(
            initial_path,
            self.nodes.get("end").unwrap().id,
            allow_revisit,
        );
        paths.len() as u32
    }
}

fn main() -> io::Result<()> {
    let graph = Graph::from_file("input.txt");
    println!("P1: Number of paths is {}", graph.count_paths(false));
    println!("P2: Number of paths is {}", graph.count_paths(true));
    Ok(())
}

#[test]
fn test1() {
    let graph1 = Graph::from_file("input-example1.txt");
    assert_eq!(graph1.count_paths(false), 10);
    assert_eq!(graph1.count_paths(true), 36);

    let graph2 = Graph::from_file("input-example2.txt");
    assert_eq!(graph2.count_paths(false), 19);
    assert_eq!(graph2.count_paths(true), 103);

    let graph3 = Graph::from_file("input-example3.txt");
    assert_eq!(graph3.count_paths(false), 226);
    assert_eq!(graph3.count_paths(true), 3509);
}
