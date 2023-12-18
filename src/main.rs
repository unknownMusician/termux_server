use std::{
    net::{
        TcpListener,
        TcpStream
    },
    io::{
        // BufReader,
        // BufRead,
        Write, BufReader, BufRead,
    },
    fs,
    thread,
    time::{
        Duration,
        SystemTime,
        UNIX_EPOCH
    },
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8987").unwrap();

    for stream in listener.incoming() {
        let Ok(stream) = stream else {
            eprintln!("[{}] Failed to establish connection.", get_time());
            thread::sleep(Duration::from_secs(2));
            continue;
        };

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request_iter = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty());

    for _line in http_request_iter {
        
    }

    const SEPARATOR: &str = "\r\n";
    let status_line = "HTTP/1.1 200 OK";

    let Ok(contents) = fs::read_to_string("content/hello.html") else {
        eprintln!("[{}] Failed to read local files.", get_time());
        return;
    };

    let length = contents.len();

    let response = format!("{status_line}{SEPARATOR}Content-Length: {length}{SEPARATOR}{SEPARATOR}{contents}");

    if let Err(error) = stream.write_all(response.as_bytes()) {
        eprintln!("[{}] Error while writing to buffer: {}", get_time(), error);
        return;
    }

    println!("[{}] Answered with hello.html.", get_time());
}

fn get_time() -> String {
    let mut since_the_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let timezone_offset = Duration::from_secs(60 * 60 * 2);

    since_the_epoch += timezone_offset;
    
    let total_seconds = since_the_epoch.as_secs();
    let seconds_today = total_seconds % (60 * 60 * 24);
    let seconds = seconds_today % 60;

    let minutes_today = seconds_today / 60;
    let minutes = minutes_today % 60;
    let hours = minutes_today / 60;

    format!("{hours:0>2}:{minutes:0>2}:{seconds:0>2}")
}