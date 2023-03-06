#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_imports)]
use csv::Writer;

pub struct CsvHeaders {
    pub name: String,
    pub age: u8,
    pub city: String,
}
    
impl CsvHeaders {
    pub fn new(name: String, age: u8, city: String) -> CsvHeaders {
        CsvHeaders { name, age, city }
    }

    pub fn print_name(&self) {
        println!("Name: {}", self.name);
    }

    pub fn print_age(&self) {
        println!("Age: {}", self.age);
    }

    pub fn print_details(&self) {
        println!("Name: {}, Age: {}, City: {}", self.name, self.age, self.city);
    }

    pub fn open_csv(&self) -> Writer<std::fs::File> {
        let mut wtr = Writer::from_path("person.csv").unwrap();
        wtr.write_record(&["Name", "Age", "City"]).unwrap();
        wtr
    }

    pub fn write_row(&self, wtr: &mut Writer<std::fs::File>) {
        wtr.write_record(&[self.name.to_owned(), self.age.to_string(), self.city.to_owned()]).unwrap();
    }
}