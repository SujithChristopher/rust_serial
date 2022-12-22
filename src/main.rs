#[allow(unused_imports)]
#[allow(unused_variables)]
#[allow(dead_code)]

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use csv::Writer;

struct CsvHeader{
    name: String, 
    age: u8,
    height: f32,
    weight: f32,
}

impl CsvHeader{
    fn new() -> CsvHeader{
        CsvHeader{
            name: String::from(""),
            age: 0,
            height: 0.0,
            weight: 0.0,
        }
    }

    fn print_name(&self){
        println!("Name: {}", self.name);
    }

    fn print_age(&self){
        println!("Age: {}", self.age);
    }

    fn open_csv_file(&self, mut file_name: String) -> File{
        file_name = file_name + ".csv";
        println!("Opening file with: {}", file_name);
        let path = Path::new(&file_name);
        let display = path.display();

        // open file for writing data
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };
        file
    }
}

fn main() {
    println!("Hello, world!");
    let mut csv_header = CsvHeader::new();
    csv_header.name = String::from("John");
    csv_header.age = 20;
    csv_header.print_name();
    csv_header.print_age();
    let file = csv_header.open_csv_file(String::from("test"));
}
