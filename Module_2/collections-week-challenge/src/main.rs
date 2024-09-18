use petgraph::algo::dijkstra;
use petgraph::dot::Dot;
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::Direction;
use std::fmt;
use std::fs::File;
use std::io::Write;

#[derive(Debug)]
struct Fighter {
    name: String,
    reach: f32,
}
/*
This is a bit like the following Python code:

class Fighter:
    def __init__(self, name):
        self.name = name
*/
impl Fighter {
    fn new(name: &str, reach: f32) -> Self {
        Self {
            name: name.to_string(),
            reach,
        }
    }
}

impl fmt::Display for Fighter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn add_edge(graph: &mut UnGraph<&Fighter, f32>, nodes: &[NodeIndex], a: usize, b: usize) {
    graph.add_edge(nodes[a], nodes[b], 1.0);
}

fn shortest_path(graph: &UnGraph<&Fighter, f32>, fighter_nodes: Vec<NodeIndex>) {
    let start_node = fighter_nodes[1];
    let end_node = fighter_nodes[4];
    let node_map = dijkstra(&graph, start_node, Some(end_node), |e| *e.weight());

    if let Some(distance) = node_map.get(&end_node) {
        println!(
            "The shortest distance from {:?} to {:?} is {} fight(s)",
            graph[start_node].name, graph[end_node].name, distance
        );
    } else {
        println!(
            "No route found from {:?} to {:?}.",
            graph[start_node], graph[end_node]
        );
    }
}

fn save_graph_dot(graph: &UnGraph<&Fighter, f32>) {
    let dot = Dot::new(&graph);
    let mut file = File::create("fighter_graph.dot").expect("Unable to create file");
    write!(file, "{:?}", dot).expect("Unable to write data");

    println!("DOT file generated: fighter_graph.dot");
    println!("To generate an image, run: dot -Tpng fighter_graph.dot -o fighter_graph.png");
}

fn main() {
    let mut graph = UnGraph::new_undirected();

    let fighters = [
        Fighter::new("Dustin Poirier", 0.8),
        Fighter::new("Khabib Nurmagomedov", 1.0),
        Fighter::new("Jose Aldo", 0.93),
        Fighter::new("Conor McGregor", 0.78),
        Fighter::new("Nate Diaz", 0.98),
    ];

    let fighter_nodes: Vec<NodeIndex> = fighters
        .iter()
        .map(|fighter| graph.add_node(fighter))
        .collect();

    add_edge(&mut graph, &fighter_nodes, 0, 1); // Dustin Poirier vs. Khabib Nurmagomedov
    add_edge(&mut graph, &fighter_nodes, 1, 3); // Khabib Nurmagomedov vs. Conor McGregor
    add_edge(&mut graph, &fighter_nodes, 3, 0); // Conor McGregor vs. Dustin Poirier
    add_edge(&mut graph, &fighter_nodes, 3, 2); // Conor McGregor vs. Jose Aldo
    add_edge(&mut graph, &fighter_nodes, 3, 4); // Conor McGregor vs. Nate Diaz
    add_edge(&mut graph, &fighter_nodes, 0, 4); // Dustin Poirier vs. Nate Diaz
    add_edge(&mut graph, &fighter_nodes, 2, 4); // Jose Aldo vs. Nate Diaz

    // 1. Calculate the degree centrality of each fighter
    for (i, &node) in fighter_nodes.iter().enumerate() {
        let name = &fighters[i].name;
        let degree = graph.edges_directed(node, Direction::Outgoing).count() as f32;
        let closeness = 1.0 / degree;
        println!("The closeness centrality of {} is {:.2}", name, closeness);
        println!("-----------------------------------------------");
    }
    println!("\n===============================================\n");

    // 2. Calculate the pagerank centrality, with reach of the fighter as a weight
    let pagerank = petgraph::algo::page_rank(&graph, 0.85, 1000);
    for (i, _) in fighter_nodes.iter().enumerate() {
        let name = &fighters[i].name;
        let rank = pagerank[i] * &fighters[i].reach;
        println!("The pagerank centrality of {} is {:.2}", name, rank);
        println!("-----------------------------------------------");
    }

    // 3. Calculate the shortest path between two fighters
    println!("\n===============================================\n");
    shortest_path(&graph, fighter_nodes);

    // 4. Generate DOT representation of a graph and save it to a file
    println!("\n===============================================\n");
    save_graph_dot(&graph);
}
