//! Graph metric utilities: rank assignment, max width, and depth.

use std::collections::{HashMap, HashSet};

use super::types::{Edge, Node};

/// Returns the effective display length of `label` after normalising line-break
/// tokens (`<br/>`, `<BR/>`, `<br>`, `<BR>`, `\n`) to actual newlines.
///
/// The length is the maximum character count across all resulting lines.
pub fn effective_label_len(label: &str) -> usize {
    if label.is_empty() {
        return 0;
    }
    let normalized = label
        .replace("<br/>", "\n")
        .replace("<BR/>", "\n")
        .replace("<br>", "\n")
        .replace("<BR>", "\n")
        .replace("\\n", "\n");
    normalized
        .split('\n')
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0)
}

/// Assigns a rank (depth level) to each node using a topological-sort-based
/// longest-path algorithm.
///
/// Cycles are handled by first removing back edges (detected via an iterative
/// DFS in node-declaration order), then ranking the remaining DAG — mirroring
/// how Mermaid itself lays out cyclic flowcharts. Disconnected nodes are
/// assigned rank `0`. Returns an empty map when `nodes` is empty.
fn rank_assign(nodes: &[Node], edges: &[Edge]) -> HashMap<String, i64> {
    if nodes.is_empty() {
        return HashMap::new();
    }
    let node_set: HashSet<&str> = nodes.iter().map(|n| n.id.as_str()).collect();
    let mut adj: HashMap<String, Vec<String>> = HashMap::new();
    for n in nodes {
        adj.insert(n.id.clone(), Vec::new());
    }
    for e in edges {
        if node_set.contains(e.from.as_str()) && node_set.contains(e.to.as_str()) {
            adj.entry(e.from.clone()).or_default().push(e.to.clone());
        }
    }

    // Pass 1: detect back edges via iterative DFS (gray = on stack, black = done),
    // visiting unvisited nodes in declaration order so the result is deterministic.
    let mut color: HashMap<String, u8> = HashMap::new(); // 0/absent=white, 1=gray, 2=black
    let mut back_edges: HashSet<(String, String)> = HashSet::new();
    for start in nodes {
        if color.get(&start.id).copied().unwrap_or(0) != 0 {
            continue;
        }
        // Stack of (node, next-neighbor-index).
        let mut stack: Vec<(String, usize)> = vec![(start.id.clone(), 0)];
        color.insert(start.id.clone(), 1);
        while let Some((cur, idx)) = stack.pop() {
            let neighbors = adj.get(&cur).cloned().unwrap_or_default();
            if idx < neighbors.len() {
                let next = neighbors[idx].clone();
                stack.push((cur.clone(), idx + 1));
                match color.get(&next).copied().unwrap_or(0) {
                    1 => {
                        back_edges.insert((cur, next));
                    }
                    0 => {
                        color.insert(next.clone(), 1);
                        stack.push((next, 0));
                    }
                    _ => {}
                }
            } else {
                color.insert(cur, 2);
            }
        }
    }

    // Pass 2: Kahn's longest-path ranking on the DAG that remains after
    // dropping the back edges.
    let mut in_degree: HashMap<String, usize> = HashMap::new();
    for n in nodes {
        in_degree.insert(n.id.clone(), 0);
    }
    for (from, tos) in &adj {
        for to in tos {
            if !back_edges.contains(&(from.clone(), to.clone())) {
                *in_degree.entry(to.clone()).or_insert(0) += 1;
            }
        }
    }
    let mut rank: HashMap<String, i64> = HashMap::new();
    let mut visited: HashSet<String> = HashSet::new();
    let mut queue: Vec<String> = Vec::new();
    for n in nodes {
        if in_degree.get(&n.id).copied().unwrap_or(0) == 0 {
            queue.push(n.id.clone());
            rank.insert(n.id.clone(), 0);
        }
    }
    while !queue.is_empty() {
        let cur = queue.remove(0);
        visited.insert(cur.clone());
        let cur_rank = *rank.get(&cur).unwrap_or(&0);
        let neighbors = adj.get(&cur).cloned().unwrap_or_default();
        for next in neighbors {
            if back_edges.contains(&(cur.clone(), next.clone())) {
                continue;
            }
            let existing = *rank.get(&next).unwrap_or(&0);
            if cur_rank + 1 > existing {
                rank.insert(next.clone(), cur_rank + 1);
            }
            if let Some(d) = in_degree.get_mut(&next) {
                *d = d.saturating_sub(1);
                if *d == 0 {
                    queue.push(next);
                }
            }
        }
    }
    for n in nodes {
        if !visited.contains(&n.id) {
            rank.entry(n.id.clone()).or_insert(0);
        }
    }
    rank
}

/// Returns the maximum number of nodes sharing the same rank (diagram width).
///
/// Returns `0` when there are no nodes.
pub fn max_width(nodes: &[Node], edges: &[Edge]) -> usize {
    if nodes.is_empty() {
        return 0;
    }
    let ranks = rank_assign(nodes, edges);
    let mut rank_count: HashMap<i64, usize> = HashMap::new();
    for r in ranks.values() {
        *rank_count.entry(*r).or_insert(0) += 1;
    }
    rank_count.values().copied().max().unwrap_or(0)
}

/// Returns the number of distinct rank levels in the diagram (diagram depth).
///
/// Returns `0` when there are no nodes.
pub fn depth(nodes: &[Node], edges: &[Edge]) -> usize {
    if nodes.is_empty() {
        return 0;
    }
    let ranks = rank_assign(nodes, edges);
    ranks.values().collect::<HashSet<_>>().len()
}
