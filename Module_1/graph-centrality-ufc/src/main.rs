use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::Direction;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
struct Fighter {
    name: String,
}
/*
This is a bit like the following Python code:

class Fighter:
    def __init__(self, name):
        self.name = name
*/
impl Fighter {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
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

// function to add fighters manually to the fighters array
fn add_fighter(fighters: &mut Vec<Fighter>){
    loop {
        println!("Enter a fighter name or type 'return' to stop:");
        let name = read_input();
        if name == "return" {
            break;
        }
        fighters.push(Fighter::new(&name));
    }
}

fn add_fights(graph: &mut UnGraph<&Fighter, f32>, nodes: &[NodeIndex]) {
    loop {
        println!("Enter the additional fights in the format 'a vs. b' or return:");
        let fights = read_input();
        if fights == "return" {
            break;
        }
        // assert the iput has form "a vs. b"
        if !fights.contains(" vs. ") {
            println!("Invalid input. Please enter the fights in format");
            continue;
        }
        let fights: Vec<&str> = fights.split(" vs. ").collect();
        let a_index = fights[0].parse::<usize>().unwrap();
        let b_index = fights[1].parse::<usize>().unwrap();

        add_edge(graph, nodes, a_index, b_index);
    }
}

// function to read from stdin
fn read_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let mut graph = UnGraph::new_undirected();

    let mut fighters = vec![
        Fighter::new("Dustin Poirier"),
        Fighter::new("Khabib Nurmagomedov"),
        Fighter::new("Jose Aldo"),
        Fighter::new("Conor McGregor"),
        Fighter::new("Nate Diaz"),
    ];

    add_fighter(&mut fighters);

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

    add_fights(&mut graph, &fighter_nodes);

    let mut closeness_hash = HashMap::new();

    for (i, &node) in fighter_nodes.iter().enumerate() {
        let name = &fighters[i].name;
        let degree = graph.edges_directed(node, Direction::Outgoing).count() as f32;
        let closeness = 1.0 / degree;

        closeness_hash.insert(name, closeness);
    }

    let mut closeness_hash: Vec<_> = closeness_hash.into_iter().collect();
    closeness_hash.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    for (i, (name, closeness)) in closeness_hash.iter().enumerate() {
        println!("{} has a closeness centrality of {:.2}", name, closeness);
        match i {
            0 => println!(
                    "{} has the lowest centrality because he has fought with all other fighters in the network. In this context, a lower centrality value means a higher number of fights.",
                    name),
            _ if i == closeness_hash.len() - 1 => println!(
                    "{} has the highest centrality as they have fought with the least number of fighters.",
                    name
                ),
            _ => {}
        }
        println!("-----------------");
    }

}
