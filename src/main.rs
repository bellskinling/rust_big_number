extern crate big_number;
use big_number::file_operation;
use std::fs::OpenOptions;
use std::io::Read;

fn main() {
    let file_name = "/tmp/file_operation_output.txt";
    let mut buffer:Vec<u8> = Vec::new();
    file_operation::read(file_name, &mut buffer);
    if let Ok(s) = String::from_utf8(buffer) {
        println!("{}", s);
    }
}
