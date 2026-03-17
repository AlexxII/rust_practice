use rust_practice::iterators::window_iterator::Window;

fn main() {
    let data = vec![1,2,3,4,5];
    let window = Window::new(data.into_iter(), 2);

    for w in window {
        println!("{:?}", w);
    }
}
