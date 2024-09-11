use std::collections::HashMap;

use daggy::{Dag, Walker};
use petgraph::{
    visit::{EdgeRef, IntoEdgesDirected},
    Direction,
};

pub fn max_path_sum(dag: &Dag<usize, ()>) -> usize {
    let mut max_sum_by_node = HashMap::new();

    petgraph::visit::Topo::new(dag.graph())
        .iter(dag.graph())
        .for_each(|node| {
            let max_parent = dag
                .edges_directed(node, Direction::Incoming)
                .map(|edge| max_sum_by_node[&edge.source()])
                .max()
                .unwrap_or(0);
            max_sum_by_node.insert(node, max_parent + dag[node]);
        });

    *max_sum_by_node.values().max().unwrap()
}

pub fn build_dag(text: &str) -> Dag<usize, ()> {
    let mut dag = Dag::new();
    let mut last_level = vec![];
    for line in text.lines() {
        let numbers = line
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        if last_level.is_empty() {
            assert!(numbers.len() == 1);
            let id = dag.add_node(numbers[0]);
            last_level = vec![id];
        } else {
            let mut new_level = vec![];
            assert!(numbers.len() == last_level.len() + 1);
            for (index, number) in numbers.into_iter().enumerate() {
                let id = dag.add_node(number);
                if index < last_level.len() {
                    dag.add_edge(last_level[index], id, ()).unwrap();
                }
                if index > 0 {
                    dag.add_edge(last_level[index - 1], id, ()).unwrap();
                }
                new_level.push(id);
            }
            last_level = new_level;
        }
    }
    dag
}
