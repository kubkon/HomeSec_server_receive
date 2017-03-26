extern crate byteorder;
extern crate chrono;

use byteorder::{ReadBytesExt,BigEndian};
use chrono::{DateTime,UTC};
use std::io::Cursor;
use std::net::UdpSocket;

fn main() {
    println!("Listening for packets...");
    loop {
        let socket = UdpSocket::bind("192.168.0.12:8888").expect("couldn't bind to address");
        let mut buf = [0; 4];
        socket.recv_from(&mut buf).expect("didn't receive any data");
        let now: DateTime<UTC> = UTC::now();

        match buf[0] {
            0x1 => { // reed
                let state = buf[1];
                let mut raw_value = Cursor::new(vec![buf[2] & 0x3, buf[3]]);
                let value = raw_value.read_u16::<BigEndian>().unwrap();
                match state {
                    0x1 => println!("Door open at {}; pin value={}", now, value),
                    0x2 => println!("Door close at {}; pin value={}", now, value),
                    _ => println!("Unknown reed state received"),
                }
            },
            0x2 => { // pir
                match buf[1] {
                    0x1 => println!("Motion detected at {}", now),
                    0x2 => println!("Motion ended at {}", now),
                    _ => println!("Unknown PIR state received"),
                }
            },
            _ => println!("Unknown message received"),
        }
    }
}
