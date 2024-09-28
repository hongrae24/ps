pub fn find(tree: &mut Vec<usize>, u: usize) -> usize {
    if tree[u] == u {
        return u;
    } else {
        tree[u] = find(tree, tree[u]);
        return tree[u];
    }
}

fn merge(tree: &mut Vec<usize>, u: usize, v: usize) {
    let u = find(tree, u);
    let v = find(tree, v);
    if u == v {
        return;
    }
    tree[u] = v;
}