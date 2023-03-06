// allow unused code for now
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_imports)]
use csv::Writer;
use serialport::SerialPortBuilder;

struct CsvHeaders {
    name: String,
    age: u8,
    city: String,
}
    


impl CsvHeaders {
    fn new(name: String, age: u8, city: String) -> CsvHeaders {
        CsvHeaders { name, age, city }
    }

    fn print_name(&self) {
        println!("Name: {}", self.name);
    }

    fn print_age(&self) {
        println!("Age: {}", self.age);
    }

    fn print_details(&self) {
        println!("Name: {}, Age: {}, City: {}", self.name, self.age, self.city);
    }

    fn open_csv(&self) -> Writer<std::fs::File> {
        let mut wtr = Writer::from_path("person.csv").unwrap();
        wtr.write_record(&["Name", "Age", "City"]).unwrap();
        wtr
    }

    fn write_row(&self, wtr: &mut Writer<std::fs::File>) {
        wtr.write_record(&[self.name.to_owned(), self.age.to_string(), self.city.to_owned()]).unwrap();
    }
}

fn main() {
    let mut person = CsvHeaders::new("John".to_string(), 25, "New York".to_string());
    person.print_name();
    person.print_age();
    person.print_details();

    let mut wtr = person.open_csv();
    person.write_row(&mut wtr);

    for i in 0..100 {
        person.name = format!("John {}", i);
        person.age = i;
        person.write_row(&mut wtr);
    }

    let data_bits = serialport::DataBits::Eight;
    let flow_control = serialport::FlowControl::None;
    let parity = serialport::Parity::None;
    let stop_bits = serialport::StopBits::One;
    let timeout = std::time::Duration::from_millis(500);

    // list all available ports
    let ports = serialport::available_ports().unwrap();
    for port in ports {
        println!("Port: {:?}", port);
    }

    let mut port = serialport::new("COM5", 115200)
        .data_bits(data_bits)
        .flow_control(flow_control)
        .parity(parity)
        .stop_bits(stop_bits)
        .timeout(timeout)
        .open()
        .unwrap();

    let mut buf: [u8; 1] = [0; 1];
    loop {
        match port.read(&mut buf) {
            Ok(t) => {
                if t > 0 {
                    println!("Read: {}", buf[0]);
                }
            }
            Err(e) => println!("Error: {}", e),
        }
    }


}


