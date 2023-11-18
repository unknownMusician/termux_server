use std::{
    net::{
        TcpListener,
        TcpStream
    },
    io::{
        // BufReader,
        // BufRead,
        Write,
    },
    fs
};

fn main() {
    let listener = TcpListener::bind("192.168.0.200:8987").unwrap();

    for stream in listener.incoming() {
        let Ok(stream) = stream else {
            eprintln!("Failed to establish connection.");
            continue;
        };

        handle_connection(stream);
    }

    println!("Hello, world!");
}

fn handle_connection(mut stream: TcpStream) {
    // let buf_reader = BufReader::new(&mut stream);
    // let http_request = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect::<Vec<_>>();

    const SEPARATOR: &str = "\r\n";
    let status_line = "HTTP/1.1 200 OK";

    let Ok(contents) = fs::read_to_string("content/hello.html") else {
        eprintln!("Failed to read local files.");
        return;
    };

    let length = contents.len();

    let response = format!("{status_line}{SEPARATOR}Content-Length: {length}{SEPARATOR}{SEPARATOR}{contents}");

    if let Err(error) = stream.write_all(response.as_bytes()) {
        eprintln!("Error while writing to buffer: {error}");
        return;
    }

    println!("Answered with hello.html.")
}
