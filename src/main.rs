// allow unused code for now
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_imports)]
use csv::Writer;
use serialport::SerialPortBuilder;

mod csv_support;

fn main() {
    let mut person = csv_support::CsvHeaders::new("John".to_string(), 25, "New York".to_string());
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


