use std::collections::BinaryHeap;
use std::cmp::Reverse;
// adjacency list with PQ: O(nlogn)
fn dijkstra_pq(start: usize, adj_list: &Vec<Vec<(usize, usize)>>, adj_mat: &Vec<Vec<usize>>, distance: &mut Vec<usize>, parent: &mut Vec<Vec<usize>>) {
    distance[start] = 0;
    let mut pq = BinaryHeap::new();
    pq.push((Reverse(0), start));
    while let Some((Reverse(d), node)) = pq.pop() {
        if distance[node] < d { continue }
        for &(next, nd) in &adj_list[node] {
            if adj_mat[node][next] == usize::MAX { continue }
            let dst = d + nd;
            if dst < distance[next] {
                distance[next] = dst;
                pq.push((Reverse(dst), next));
                parent[next] = vec![node];
            } else if dst == distance[next] {
                parent[next].push(node);
            }
        }
    }
}

unsafe fn solve() {
    loop {
        input!(n: usize, m: usize);
        if n == 0 && m == 0 { return }
        input! {
            s: usize, d: usize,
            edges: [(usize, usize, usize); m],
        };
        let mut adj_list = vec![vec![]; n];
        for &(u, v, p) in &edges {
            adj_list[u].push((v, p));
        }
        for i in 0..n {
            adj_list[i].sort_unstable_by_key(|&(v, p)| (p, v));
        }
        let mut adj_mat = vec![vec![usize::MAX; n]; n];
        for i in 0..n {
            adj_mat[i][i] = 0;
        }
        for &(u, v, p) in &edges {
            adj_mat[u][v] = p;
        }
        let mut distance = vec![usize::MAX; n];
        let mut parent = vec![vec![]; n];
        dijkstra_pq(s, &adj_list, &adj_mat, &mut distance, &mut parent);
        let min_dst = distance[d];
        if min_dst == usize::MAX {
            println!("-1");
            continue;
        }
        let mut stack = vec![d];
        while let Some(i) = stack.pop() {
            while let Some(j) = parent[i].pop() {
                adj_mat[j][i] = usize::MAX;
                stack.push(j);
            }
        }
        let mut distance = vec![usize::MAX; n];
        dijkstra_pq(s, &adj_list, &adj_mat, &mut distance, &mut parent);
        let ans = distance[d];
        if ans == usize::MAX { println!("-1") }
        else { println!("{ans}") }
    }
}
