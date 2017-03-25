use std::net::UdpSocket;
use std::time::SystemTime;

fn main() {
    println!("Listening for packets...");
    loop {
        let socket = UdpSocket::bind("192.168.0.12:8888").expect("couldn't bind to address");
        let mut buf = [0; 2];
        socket.recv_from(&mut buf).expect("didn't receive any data");
        let now = SystemTime::now();

        match buf[0] {
            0x1 => { // reed
                match buf[1] {
                    0x1 => println!("Door open at {:?}", now),
                    0x2 => println!("Door close at {:?}", now),
                    _ => println!("Unknown reed state received"),
                }
            },
            0x2 => { // pir
                match buf[1] {
                    0x1 => println!("Motion detected at {:?}", now),
                    0x2 => println!("Motion ended at {:?}", now),
                    _ => println!("Unknown PIR state received"),
                }
            },
            _ => println!("Unknown message received"),
        }
    }
}
