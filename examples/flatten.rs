use rust_practice::iterators::flatten_iterator::Flatten;

fn main() {
    let data = vec![vec![1, 2], vec![3, 4], vec![5]];

    let flatten = Flatten::new(data.into_iter());

    for x in flatten {
        println!("{x}");
    }
}
