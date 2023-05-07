use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878");
    if let Err(e) = &listener {
        eprintln!("Server creation failed: {}", e);
    }
    let listener = listener.unwrap();

    for stream in listener.incoming() {
        if let Err(e) = &stream {
            eprintln!("Connection failed: {}", e);
        }
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    if let Some(Ok(request_line)) = buf_reader.lines().next() {
        let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
            ("HTTP/1.1 200 OK", "hello.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };
        let contents =
            fs::read_to_string(filename).unwrap_or("Error: file doesn't exist!".to_string());
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-length: {length}\r\n\r\n{contents}");
        
        match stream.write_all(response.as_bytes()) {
            Ok(_) => (),
            Err(e) => eprintln!("Something bad happened! {}", e),
        };
    }
}
