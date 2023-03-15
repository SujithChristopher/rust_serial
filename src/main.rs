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

    let mut header1 = 0;
    let mut header2 = 0;
    let mut databytes = 0;
    loop {
        // read from serial port
        header1 = match port.read(&mut buf) {
            Ok(t) => {
                if t > 0 {
                    buf[0]
                } else {
                    0
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                0
            }
        };
        // read from serial port
        header2 = match port.read(&mut buf) {
            Ok(t) => {
                if t > 0 {
                    buf[0]
                } else {
                    0
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                0
            }
        };

        if header1 == 255 && header2 == 255 {
            // read from serial port
            databytes = match port.read(&mut buf) {
                Ok(t) => {
                    if t > 0 {
                        buf[0]
                    } else {
                        0
                    }
                }
                Err(e) => {
                    println!("Error: {}", e);
                    0
                }
            };

            // define a vector to hold the values size of val3
            let mut vec = Vec::with_capacity(databytes as usize);
            
            for i in 0..databytes {
                // read from serial port
                let val = match port.read(&mut buf) {
                    Ok(t) => {
                        if t > 0 {
                            buf[0]
                        } else {
                            0
                        }
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                        0
                    }
                };

                vec.push(val);
            }
            // print the vector 0 to 3
            // println!("Read: {:?}", vec);

            // convert first 4 bytes to float
            let mut bytes = [0u8; 4];
            bytes[0] = vec[0];
            bytes[1] = vec[1];
            bytes[2] = vec[2];
            bytes[3] = vec[3];
            let f = f32::from_le_bytes(bytes);
            println!("Float: {}", f);
        }

    }
}
