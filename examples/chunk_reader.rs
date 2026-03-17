use std::fs::File;
use std::io;

use rust_practice::iterators::chunk_reader::ChunkReader;

fn main() -> Result<(), io::Error> {
    let file = File::open("data.txt")?;
    let reader = ChunkReader::new(file, 4);

    for chunk in reader {
        println!("{:?}", chunk);
    }
    Ok(())
}
