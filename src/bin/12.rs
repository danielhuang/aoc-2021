

use aoc_2021::input;
use defaultmap::DefaultHashMap;
use itertools::Itertools;
use petgraph::{algo::all_simple_paths, graphmap::UnGraphMap};

fn main() {
    let input = input!("12");
    dbg!(&input);

    let edges = input
        .lines()
        .map(|x| x.split('-').collect_tuple().unwrap())
        .map(|(a, b)| (a, b));

    let mut graph = UnGraphMap::<_, ()>::from_edges(edges);

    let from = graph.add_node("start");
    let to = graph.add_node("end");
    let ways = all_simple_paths::<Vec<_>, _>(&graph, from, to, 0, None).collect_vec();

    dbg!(&graph);

    dbg!(&ways);
    dbg!(&ways.len());
    dbg!(search(&graph, "start", "end", &["start"], false));
}

fn search<'a>(
    graph: &'a UnGraphMap<&str, ()>,
    from: &'a str,
    to: &'a str,
    stack: &[&'a str],
    _visited_small_twice: bool,
) -> usize {
    if from == to {
        return 1;
    }

    let mut count = 0;
    for adj in graph.neighbors(from) {
        if (adj.chars().next().unwrap().is_uppercase()
            || if visited_twice(stack) {
                !stack.contains(&adj)
            } else {
                stack.iter().filter(|&&x| x == adj).count() < 2
            })
            && adj != "start"
        {
            let mut new_stack = stack.to_owned();
            new_stack.push(adj);

            count += search(graph, adj, to, &new_stack, !stack.contains(&adj));
        }
    }

    count
}

fn visited_twice(stack: &[&str]) -> bool {
    let mut map = DefaultHashMap::new(0);
    for f in stack {
        if !f.chars().next().unwrap().is_uppercase() {
            map[*f] += 1;
        }
    }
    map.values().max().unwrap() > &1
}
