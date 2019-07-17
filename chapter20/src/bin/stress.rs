use std::io::prelude::*;
use std::net::TcpStream;
use std::sync::mpsc;
use std::thread;
use std::time::Instant;

fn experiment() -> mpsc::Receiver<u128> {
    let (tx, rx) = mpsc::channel();

    for _ in 0..16 {
        let tx = tx.clone();
        thread::spawn(move || {
            for _ in 0..100 {
                let mut stream = TcpStream::connect("localhost:7878").unwrap();

                let start = Instant::now();
                stream.write(b"GET /slow HTTP/1.1\r\n").unwrap();
                let mut buffer = [0; 512];
                stream.read(&mut buffer).unwrap();
                let end = Instant::now();
                let took = end - start;
                println!("{:?}", took.as_millis());

                tx.send(took.as_millis()).unwrap();
            }
        });
    }

    rx
}

fn main() {
    let rx = experiment();

    println!("collecting");
    let (len, sum) = rx.iter().fold((0, 0), |(len, sum), v| (len + 1, sum + v));
    println!("len {} sum {} mean {}", len, sum, sum as f64 / len as f64);
}
