use serde_json::Value;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;

pub fn count_provinces() -> String {
    // 读取 JSON 文件
    let data = fs::read_to_string("district.json").expect("Unable to read file");
    let graph_input: Value = serde_json::from_str(&data).expect("Unable to parse JSON");

    // 收集每个大区的强连通分量数量
    let mut components_counts: Vec<String> = Vec::new();

    let graph_input: HashMap<String, HashMap<String, Vec<String>>> =
        serde_json::from_value(graph_input).unwrap();
    let mut scc_counts = calculate_scc_for_regions(graph_input);

    for (region, count) in scc_counts.iter() {
        println!(
            "Region {} has {} strongly connected components.",
            region, count
        );
    }
    scc_counts
        .values()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn dfs(node: &str, graph: &HashMap<String, Vec<String>>, visited: &mut HashSet<String>) {
    visited.insert(node.to_string());
    if let Some(neighbors) = graph.get(node) {
        for neighbor in neighbors {
            if !visited.contains(neighbor) {
                dfs(neighbor, graph, visited);
            }
        }
    }
}

fn count_connected_components(graph: &HashMap<String, Vec<String>>) -> usize {
    let mut visited = HashSet::new();
    let mut count = 0;

    for node in graph.keys() {
        if !visited.contains(node) {
            dfs(node, graph, &mut visited);
            count += 1;
        }
    }

    count
}

fn calculate_scc_for_regions(
    graph_input: HashMap<String, HashMap<String, Vec<String>>>,
) -> BTreeMap<String, usize> {
    let mut result = BTreeMap::new();

    for (region, cities) in graph_input {
        let mut region_graph = HashMap::new();

        for (city, neighbors) in cities {
            region_graph
                .entry(city.clone())
                .or_insert_with(Vec::new)
                .extend(neighbors.clone());
            for neighbor in neighbors {
                region_graph
                    .entry(neighbor.clone())
                    .or_insert_with(Vec::new)
                    .push(city.clone());
            }
        }

        let scc_count = count_connected_components(&region_graph);
        result.insert(region, scc_count);
    }

    result
}
