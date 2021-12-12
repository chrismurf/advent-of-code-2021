use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

type NodeId = u16;

#[derive(Debug)]
struct Node {
    name: String,
    revisitable: bool,
}

#[derive(Debug)]
struct Graph {
    start: Option<NodeId>,
    end: Option<NodeId>,
    nodes: Vec<Node>,
    edges: HashMap<NodeId, HashSet<NodeId>>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            start: None,
            end: None,
            nodes: Vec::new(),
            edges: HashMap::new(),
        }
    }

    fn from_file(path: &str) -> Self {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let mut graph: Graph = Graph::new();
        for line in reader.lines() {
            let names: Vec<String> = line.unwrap().split("-").map(|x| x.to_owned()).collect();
            graph.add_edge(
                names.get(0).unwrap(),
                names.get(1).unwrap()
            );
        }
        graph
    }

    fn ensure_node_exists(&mut self, name: &str) -> NodeId {
        let node_id: NodeId;
        match self.nodes.iter().position(|n| n.name == name) {
            Some(position) => {
                return position as NodeId;
            },

            None => {
                node_id = self.nodes.len() as NodeId;
                self.nodes.push(Node {
                    name: name.to_owned(),
                    revisitable: name.to_ascii_uppercase() == name
                });
                if name == "start" { self.start = Some(node_id)};
                if name == "end" { self.end = Some(node_id)};
                return node_id;
            }
        }
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
        prefix: Vec<NodeId>,
        destination: NodeId,
        allow_revisit: bool,
    ) -> Vec<Vec<NodeId>> {
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
                let next_node = self.nodes.get(next_id as usize).unwrap();
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

    fn count_paths(&self, allow_revisit: bool) -> usize {
        let initial_path: Vec<NodeId> = vec![self.start.unwrap()];
        let paths = self.generate_paths_via(
            initial_path,
            self.end.unwrap(),
            allow_revisit,
        );
        paths.len()
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
