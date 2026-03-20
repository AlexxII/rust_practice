use rust_practice::iterators::tree::Node;

fn main() {
    let mut root = Node::new("A");

    let mut b = Node::new("B");
    b.add_child(Node::new("D"));
    b.add_child(Node::new("E"));

    let c = Node::new("C");

    root.add_child(b);
    root.add_child(c);
    let mut root_copy = root.clone();
    assert!(root.eq(&root_copy));
    println!("{}", root == root_copy);
    root_copy.add_child(Node::new("Q"));
    println!("root_copy -  {:?}", root_copy);
    println!("root - {:?}", root);
    println!("{}", root == root_copy);

    let mut root_e = Node::new("Z");
    root_e.add_child(Node::new("X"));
    let json = serde_json::to_string_pretty(&root).unwrap();
    println!("{}", json);
    //
    // let dfs: Vec<_> = root.dfs().map(|n| n.value).collect();
    // println!("{:?}", dfs);
    //
    // let bfs: Vec<_> = root.bfs().map(|n| n.value).collect();
    // println!("{:?}", bfs);
}
