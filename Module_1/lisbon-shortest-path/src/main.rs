use petgraph::algo::dijkstra;
use petgraph::prelude::*;
use std::collections::HashMap;

fn read_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let mut graph = Graph::<String, u32, Undirected>::new_undirected();
    let mut node_map = HashMap::new();

    let belem_tower = graph.add_node("Belem Tower".to_string());
    let monastery = graph.add_node("Jerónimos Monastery".to_string());
    let lx_factory = graph.add_node("LX Factory".to_string());
    let commerce_square = graph.add_node("Commerce Square".to_string());
    let lisbon_cathedral = graph.add_node("Lisbon Cathedral".to_string());

    node_map.insert("Belem Tower".to_string(), belem_tower);
    node_map.insert("Jerónimos Monastery".to_string(), monastery);
    node_map.insert("LX Factory".to_string(), lx_factory);
    node_map.insert("Commerce Square".to_string(), commerce_square);
    node_map.insert("Lisbon Cathedral".to_string(), lisbon_cathedral);

    loop {
        println!("Do you want to add nodes? y/n");
        let input = read_input();
        if input == "y" {
            println!("Enter the name of the node: ");
            let node_name = read_input();
            let new_node = graph.add_node(node_name.clone());
            node_map.insert(node_name, new_node);
        } else if input == "n" {
            break;
        } else {
            println!("Invalid input. Please enter y or n.");
        }
    }

    graph.extend_with_edges([
        (belem_tower, monastery, 1),
        (belem_tower, lx_factory, 3),
        (belem_tower, commerce_square, 7),
        (monastery, lx_factory, 3),
        (monastery, commerce_square, 6),
        (lx_factory, commerce_square, 5),
        (commerce_square, lisbon_cathedral, 1),
    ]);

    loop {
        println!("Do you want to add edges? y/n");
        let input = read_input();
        if input == "y" {
            println!("Enter the name of the first node: ");
            let first_node = read_input();
            println!("Enter the name of the second node: ");
            let second_node = read_input();
            println!("Enter the distance between the nodes: ");
            let distance = read_input().parse::<u32>().unwrap();
            let first_node_index = node_map.get(&first_node).unwrap();
            let second_node_index = node_map.get(&second_node).unwrap();
            graph.add_edge(*first_node_index, *second_node_index, distance);
        } else if input == "n" {
            break;
        } else {
            println!("Invalid input. Please enter y or n.");
        }
    }

    println!("The landmarks are: ");
    for node in graph.node_indices() {
        println!("{}", graph[node]);
    }

    println!("Enter the starting node: ");
    let start_node = NodeIndex::new(read_input().parse::<usize>().unwrap());

    println!("Enter the ending node: ");
    let end_node = NodeIndex::new(read_input().parse::<usize>().unwrap());

    let node_map = dijkstra(&graph, start_node, Some(end_node), |e| *e.weight());

    if let Some(distance) = node_map.get(&end_node) {
        println!(
            "The shortest distance from {:?} to {:?} is {} km",
            graph[start_node], graph[end_node], distance
        );
    } else {
        println!(
            "No route found from {:?} to {:?}.",
            graph[start_node], graph[end_node]
        );
    }
}
