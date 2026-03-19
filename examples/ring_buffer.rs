use rust_practice::iterators::ring_buffer::RingBuffer;

fn main() {
    let mut buffer: RingBuffer<i32> = RingBuffer::new(4);
    buffer.push(1);
    buffer.push(2);
    buffer.push(3);
    buffer.pop();
    buffer.push(4);
    buffer.push(5);
    println!("{:?}", buffer);
    println!("{}", buffer);
    let mut buf_copy = buffer.clone();
    buf_copy.push(9);
    println!("{}", buffer == buf_copy);
    // for i in buffer.iter() {
    //     println!("{i}");
    // }
}
