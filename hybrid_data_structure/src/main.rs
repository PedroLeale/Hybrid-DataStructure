use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Write};
use std::path::Path;

mod hybrid_data_structure;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    
}
