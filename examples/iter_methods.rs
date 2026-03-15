use rand::random;
use std::iter::{from_fn, successors};

fn main() {
    let fib_vec: Vec<_> = fibonacci().take(10).collect();
    println!("{:?}", fib_vec);

    drain();
}

#[allow(dead_code)]
fn drain() {
    let mut outer = "Earth".to_string();
    let inner = String::from_iter(outer.drain(1..4));
    println!("{outer}");
    println!("{inner}");
}

#[allow(dead_code)]
fn gen_vec() -> Vec<f64> {
    from_fn(|| Some((random::<f64>() - random::<f64>()).abs()))
        .take(10)
        .collect()
}

#[allow(dead_code)]
fn succ() {
    let ass_vec: Vec<_> = successors(Some(1_u64), |n| n.checked_mul(10)).collect();
    println!("{:?}", ass_vec);
}

fn fibonacci() -> impl Iterator<Item = usize> {
    let mut state = (0, 1);
    from_fn(move || {
        state = (state.1, state.0 + state.1);
        Some(state.0)
    })
}
