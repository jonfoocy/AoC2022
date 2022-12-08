#![feature(buf_read_has_data_left)]
use std::fs::File;
use std::io::BufReader;

pub fn print_hello() {
    println!("Hello, world!");
}

pub fn get_file_reader(path: &String) -> BufReader<File> {
    let f: File = File::open(path).unwrap();
    let reader: BufReader<File> = BufReader::new(f);

    reader
}
