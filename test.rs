use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::create("foo.txt").unwrap();
    file.write_all(b"Hello, world!");

    let mut file2 = File::open("foo.txt").unwrap();
    let mut contents = String::new();
    file2.read_to_string(&mut contents);
    println!("cont: {}", contents);
}