use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    // buffer with 1024 bytes to read data from the TcpStream
    let mut buffer = [0; 1024];

    // read data from the stream and stores it in the buffer. This will block the current thread while waiting for data
    loop {
        // TcpStream.read() actually returns exactly how many bytes were written, we need to output only that many bytes.
        let bytes_read = stream
            .read(&mut buffer)
            .expect("Failed to read from client!");

        // converts the data in the buffer to a UTF8 encoded string
        let request = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Received request: {}", request);

        // Prepares a response and converts it to bytes to be returned as a row of bytes.
        let built_response = format!("{request}");

        let response = built_response.as_bytes();

        // Write to the client
        stream.write(response).expect("Failed to write response!");
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Failed to bind to address");
    println!("Server listening on 127.0.0.1:7878");
    // Loop over incoming messages
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                // eprintln! is a stderr - standard error stream used by the OS
                eprintln!("Failed to establish connection: {}", e);
            }
        }
    }
}
