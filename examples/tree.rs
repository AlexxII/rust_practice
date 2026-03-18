use rust_practice::iterators::tree::Node;

fn main() {
    let mut root = Node::new("A");

    let mut b = Node::new("B");
    b.add_child(Node::new("D"));
    b.add_child(Node::new("E"));

    let c = Node::new("C");

    root.add_child(b);
    root.add_child(c);
    println!("{:?}", root);

    let dfs: Vec<_> = root.dfs().map(|n| n.value).collect();
    println!("{:?}", dfs);

    let bfs: Vec<_> = root.bfs().map(|n| n.value).collect();
    println!("{:?}", bfs);
}
