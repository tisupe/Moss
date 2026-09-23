use std::thread;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use flate2::Compression;
use flate2::write::GzEncoder;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                thread::spawn(move || {
                    loop {
                        let mut buffer = [0; 1024];
                        let bytes_read = stream.read(&mut buffer).unwrap();
                        if bytes_read == 0 {
                            break;
                        } else {
                            let read_buffer = String::from_utf8_lossy(&buffer);
                            let lining_buffer = read_buffer.lines().next().unwrap(); //read buffer gets lined in stored
                            let parts: Vec<&str> = lining_buffer.split(' ').collect(); //colleced in the 'parts' vector and spilit based on spaces then stored
                            let (headers, body) = read_buffer.split_once("\r\n\r\n").unwrap();

                            println!("\r\n[ read_buffer ]:\r\n{}", read_buffer);
                            println!("\r\n[ lining_buffer ]:\r\n{}", lining_buffer);
                            println!("\r\n[ parts ]:\r\n{} {} {}", parts[0], parts[1], parts[2]);
                            println!("\r\n[ headers, body ]:\r\n{} {}", headers, body);

                            let mut gzip_support = false;
                            for line in read_buffer.lines() {
                                if line.starts_with("Accept-Encoding: ") {
                                    let value = line.strip_prefix("Accept-Encoding: ").unwrap();
                                    for part in value.split(',') {
                                        if part.trim() == "gzip" {
                                            gzip_support = true;
                                        } // closes if part.trim
                                    } // closes for part
                                    println!("\r\n[ gzip_support ]:\r\n{}", gzip_support);
                                } // closes "Accept-Encoding: " check
                            } // closes for

                            if parts[1] == "/" {
                                stream
                                    .write_all("HTTP/1.1 200 OK\r\n\r\n".as_bytes())
                                    .unwrap();
                            } else if parts[1].starts_with("/echo/") {
                                let echo_str = parts[1].strip_prefix("/echo/").unwrap();
                                let encoding_header = if gzip_support {
                                    "Content-Encoding: gzip\r\n"
                                } else {
                                    ""
                                };
                                let response = format!(
                                    "HTTP/1.1 200 OK\r\n{}Content-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                                    encoding_header,
                                    echo_str.len(),
                                    echo_str
                                );
                                println!("\r\n[ echo ]:\r\n{}", echo_str);
                                stream.write_all(response.as_bytes()).unwrap();
                            } else if parts[1] == "/user-agent" {
                                let mut user_agent_str = String::from("");
                                for line in read_buffer.lines() {
                                    if line.starts_with("User-Agent: ") {
                                        user_agent_str =
                                            line.strip_prefix("User-Agent: ").unwrap().to_string();
                                    }
                                }
                                let response = format!(
                                    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                                    user_agent_str.len(),
                                    user_agent_str
                                );
                                println!("\r\n[ user-agent ]:\r\n{}", user_agent_str);
                                stream.write_all(response.as_bytes()).unwrap();
                            } else if parts[0] == "POST" && parts[1].starts_with("/files/") {
                                let mut content_len: usize = 0;
                                for line in read_buffer.lines() {
                                    if line.starts_with("Content-Length: ") {
                                        content_len = line
                                            .strip_prefix("Content-Length: ")
                                            .unwrap()
                                            .parse()
                                            .unwrap();
                                    }
                                }
                                println!("\r\n[ content-agent ]:\r\n{}", content_len);

                                //cleaner real body content
                                let real_body = &body[0..content_len];
                                println!("\r\n[ real_body ]:\r\n{}", real_body);

                                //take name from real_body and make a file
                                let filename = parts[1].strip_prefix("/files/").unwrap();
                                std::fs::write(filename, real_body).unwrap(); // std::fs is headerfile for file io operations
                                stream
                                    .write_all("HTTP/1.1 201 Created\r\n\r\n".as_bytes())
                                    .unwrap();
                            } else {
                                stream
                                    .write_all("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes())
                                    .unwrap();
                            }
                            let mut close = false;
                            for line in read_buffer.lines() {
                                if line == "Connection: close" {
                                    close = true;
                                }
                            }
                            println!("\r\n[ close ]:\r\n{}", close);
                            if close == true {
                                break;
                            }
                        } // closes bytes_read else
                    } // closes loop
                }); // closes thread
            } // closes Ok
            Err(e) => {
                println!("\r\n[ error ]:\r\n{}", e);
            } // ← closes Err
        } // closes match
    } // closes for for loop
} // closes main
