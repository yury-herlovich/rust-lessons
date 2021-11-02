use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use webapp::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let (status, content) = if buffer.starts_with(b"GET / HTTP/1.1\r\n") {
        let json = "{\"msg\":\"success\"}";

        ("200 OK", json)
    } else {
        ("404 NOT FOUND", "")
    };

    let response = format!(
        "HTTP/1.1 {}\r\nContent-Type:application/json\r\n\r\n{}\r\n",
        status, content,
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
