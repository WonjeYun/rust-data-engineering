use csv::Reader;
use std::error::Error;
use std::fs::File;
use std::path::Path;

use std::collections::HashMap;
use std::io::{BufRead, BufReader};

use std::collections::LinkedList;

use petgraph::graph::{Graph, NodeIndex, UnGraph};
use std::fmt;

use petgraph::algo::kosaraju_scc;
use petgraph::graph::DiGraph;

fn read_csv(filepath: &str) -> Result<Reader<File>, Box<dyn Error>> {
    let path = Path::new(filepath);
    let file = File::open(path)?;
    let rdr = Reader::from_reader(file);
    Ok(rdr)
}

fn read_column_data(rdr: &mut Reader<File>, col_idx: usize) -> Vec<f64> {
    let mut data: Vec<f64> = Vec::new();
    for result in rdr.records() {
        let record = result.unwrap();
        let value: f64 = record[col_idx].parse().unwrap();
        data.push(value);
    }
    data
}

fn return_stats(data: &Vec<f64>, col_idx: usize) {
    let min = data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max = data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let sum: f64 = data.iter().sum();
    let mean = sum / data.len() as f64;

    println!("Statistics for column {}:", col_idx);
    println!("Minimum: {}", min);
    println!("Maximum: {}", max);
    println!("Mean: {}", mean);
}

fn read_txt(filepath: &str) -> Result<BufReader<File>, Box<dyn Error>> {
    let path = Path::new(filepath);
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    Ok(reader)
}

fn clean_word(word: &str) -> String {
    let cleaned: String = word
        .chars()
        .filter(|c| c.is_alphabetic())
        .flat_map(|c| c.to_lowercase())
        .collect();
    if cleaned.is_empty() {
        String::from("[EMPTY]")
    } else {
        cleaned
    }
}

fn count_words(reader: &mut BufReader<File>) -> HashMap<String, u32> {
    let mut word_count: HashMap<String, u32> = HashMap::new();
    for line in reader.lines() {
        let line = line.unwrap();
        for word in line.split_whitespace() {
            let cleaned_word = clean_word(word);
            let count = word_count.entry(cleaned_word).or_insert(0);
            *count += 1;
        }
    }
    word_count
}

fn print_freq_words(word_count: &HashMap<String, u32>, n: usize) {
    let mut word_count: Vec<_> = word_count.into_iter().collect();
    word_count.sort_by(|a, b| b.1.cmp(&a.1));

    println!("Top {} most frequent words:", n);
    for (word, count) in word_count.iter().take(n) {
        println!("{}: {}", word, count);
    }
}

fn print_frq_wrds_alph(word_list: LinkedList<(String, u32)>) {
    let mut sorted_list: Vec<_> = word_list.into_iter().collect();

    // Sort by frequency (descending) and keep top 10, then sort alphabetically
    sorted_list.sort_by(|a, b| b.1.cmp(&a.1));
    sorted_list.truncate(10);
    sorted_list.sort_by(|a, b| a.0.cmp(&b.0));

    // Print the top 10 most common words
    println!("Top 10 most common words (alphabetically sorted):");
    for (word, count) in sorted_list {
        println!("{}: {}", word, count);
    }
}

// Challenge 4
#[derive(Debug)]
struct Fighter {
    name: String,
}

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

fn is_fully_connected<N, E, Ty: petgraph::EdgeType>(graph: &Graph<N, E, Ty>) -> bool {
    let num_nodes = graph.node_count();
    let num_edges = graph.edge_count();
    if num_nodes <= 1 {
        return true;
    }

    if Ty::is_directed() {
        println!("Directed graph");
        num_edges == num_nodes * (num_nodes - 1)
    } else {
        println!("Undirected graph");
        num_edges == num_nodes * (num_nodes - 1) / 2
    }
}

fn main() {
    // Challenge 1
    println!("\nChallenge 1 =====================\n");
    let filepath_1 = "data.csv";
    let mut csv_data = read_csv(&filepath_1).expect("Failed to read CSV file");
    let col_idx = 1;

    let data = read_column_data(&mut csv_data, col_idx);

    if data.is_empty() {
        println!("No valid numeric data found in the specified column.");
        return ();
    }

    return_stats(&data, col_idx);

    // Challenge 2
    println!("\nChallenge 2 =====================\n");
    let filepath_2 = "text_data.txt";
    let mut txt_data = read_txt(&filepath_2).expect("Failed to read text file");
    let word_counts = count_words(&mut txt_data);

    print_freq_words(&word_counts, 10);

    // Challenge 3
    println!("\nChallenge 3 =====================\n");
    let mut word_list: LinkedList<(String, u32)> = LinkedList::new();
    for (word, count) in word_counts {
        word_list.push_back((word, count));
    }

    print_frq_wrds_alph(word_list);

    // Challenge 4
    println!("\nChallenge 4 =====================\n");
    let mut un_graph = UnGraph::new_undirected();

    let fighters = vec![
        Fighter::new("Dustin Poirier"),
        Fighter::new("Khabib Nurmagomedov"),
        Fighter::new("Jose Aldo"),
        Fighter::new("Conor McGregor"),
        Fighter::new("Nate Diaz"),
    ];

    let fighter_nodes: Vec<NodeIndex> = fighters
        .iter()
        .map(|fighter| un_graph.add_node(fighter))
        .collect();

    add_edge(&mut un_graph, &fighter_nodes, 0, 1); // Dustin Poirier vs. Khabib Nurmagomedov
    add_edge(&mut un_graph, &fighter_nodes, 1, 3); // Khabib Nurmagomedov vs. Conor McGregor
    add_edge(&mut un_graph, &fighter_nodes, 3, 0); // Conor McGregor vs. Dustin Poirier
    add_edge(&mut un_graph, &fighter_nodes, 3, 2); // Conor McGregor vs. Jose Aldo
    add_edge(&mut un_graph, &fighter_nodes, 3, 4); // Conor McGregor vs. Nate Diaz
    add_edge(&mut un_graph, &fighter_nodes, 0, 4); // Dustin Poirier vs. Nate Diaz
    add_edge(&mut un_graph, &fighter_nodes, 2, 4); // Jose Aldo vs. Nate Diaz

    if is_fully_connected(&un_graph) {
        println!("The graph is fully connected.");
    } else {
        println!("The graph is not fully connected.");
    }

    // Challenge 5
    println!("\nChallenge 5 =====================\n");
    let mut di_graph = DiGraph::<&Fighter, &str>::new();
    let di_fights = vec![
        vec![0, 1],
        vec![1, 3],
        vec![3, 0],
        vec![3, 2],
        vec![3, 4],
        vec![0, 4],
        vec![2, 4],
    ];

    for di_fight in di_fights {
        let a = di_fight[0];
        let b = di_fight[1];
        di_graph.add_node(&fighters[a]);
        di_graph.add_node(&fighters[b]);
        di_graph.add_edge(fighter_nodes[a], fighter_nodes[b], "fight");
    }

    let scc = kosaraju_scc(&di_graph);
    let largest_community = scc.iter().max_by_key(|component| component.len()).unwrap();
    println!(
        "{} nodes in largest community discovered, with: ",
        largest_community.len()
    );
    let usernames: Vec<&str> = largest_community
        .iter()
        .map(|&node_index| di_graph[node_index].name.as_str())
        .collect();
    println!("{:?}", usernames);
}
