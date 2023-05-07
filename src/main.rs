use std::net::TcpListener;

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

        

        println!("Connection established!");
    }
}
