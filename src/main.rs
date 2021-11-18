extern crate serial;

use ascii::ToAsciiChar;
use serial::prelude::*;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;
fn main() {
    const SETTINGS: serial::PortSettings = serial::PortSettings {
        baud_rate: serial::Baud115200,
        char_size: serial::Bits8,
        parity: serial::ParityNone,
        stop_bits: serial::Stop1,
        flow_control: serial::FlowNone,
    };
    let mut port = serial::open("/dev/cu.usbmodem143401").unwrap();
    port.configure(&SETTINGS).unwrap();
    port.set_timeout(Duration::from_millis(1000)).unwrap();
    let mut buf: Vec<u8> = (0..255).collect();
    loop {
        if let Ok(n) = port.read(&mut buf[..]) {
            for i in 0..n {
                let mut res = String::from("");
                if buf[i] == 2 {}
                res.push(buf[i].to_ascii_char().unwrap().as_char());
                print!("{}", &res);
                if buf[i] == 3 {
                    println!();
                }
            }
        };
        thread::sleep(Duration::from_millis(100));
    }
}
