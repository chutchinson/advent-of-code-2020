#[macro_use] extern crate lazy_static;

mod prelude;
use prelude::*;
use petgraph::prelude::*;
use petgraph::dot::Dot;

type BagGraph<'a> = Graph<&'a str, usize>;

lazy_static! {
    static ref ENTRY: Regex = Regex::new(r#"(?:(\d) )?([a-z ]+?) (?:bag)"#).unwrap();
}

fn parse(data: &str) -> Option<BagGraph> {
    let mut graph = BagGraph::new();
    let mut nodes = HashMap::<&str, NodeIndex>::new();

    for line in data.lines() {
        let mut bags = ENTRY.find_iter(line)
            .map(|rmatch| {
                let captures = ENTRY.captures(rmatch.as_str()).unwrap();
                let count = captures.get(1).map(|x| x.as_str().parse::<usize>().unwrap_or(0)).unwrap_or(0);
                let name = captures.get(2).map(|x| x.as_str()).unwrap_or("");
                (count, name)
            });
        let (_, parent) = bags.next().unwrap();
        for (count, name) in bags {
            if count > 0 {
                let parent_node = *nodes.entry(parent).or_insert_with(|| graph.add_node(parent));
                let child_node = *nodes.entry(name).or_insert_with(|| graph.add_node(name));
                graph.add_edge(parent_node, child_node, count);
            }
        }
    }

    Some(graph)
}

fn count_predecessors(graph: &BagGraph, root: NodeIndex) -> usize {
    let mut graph = graph.clone();
    graph.reverse();
    let mut walk = Dfs::new(&graph, root);
    let mut count = 0;
    while let Some(_) = walk.next(&graph) {
        count += 1;
    }
    count - 1
}

fn count_descendants(graph: &BagGraph, root: NodeIndex) -> usize {
    let mut count = 1;
    for neighbor in graph.neighbors(root) {
        let edge = graph.find_edge(root, neighbor).unwrap();
        let weight = graph.edge_weight(edge).unwrap();
        count += weight * count_descendants(graph, neighbor)
    }
    count
}

fn main() {
    let mut data = String::new();
    let _ = stdin().lock().read_to_string(&mut data);

    let graph = parse(&data).unwrap();

    let root = graph.node_indices()
        .find(|n| graph.node_weight(*n).unwrap() == &"shiny gold")
        .unwrap();

    println!("{}", count_predecessors(&graph, root));
    println!("{}", count_descendants(&graph, root) - 1);
        
    if std::env::args().any(|x| x == "dot") {
        let dot = Dot::new(&graph);
        let filename = "day7.dot";
        let mut file = std::fs::File::create(filename).unwrap();
        write!(file, "{}", dot).unwrap();
        println!("generated graph -> {}", filename);
    }
}