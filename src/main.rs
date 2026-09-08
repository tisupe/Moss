use std::io::Read;
use std::io::Write;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        let mut buffer = [0; 1024];

        match stream {
            Ok(mut stream) => {
                stream.read(&mut buffer).unwrap();

                let read_buffer = String::from_utf8_lossy(&buffer);
                let lining_buffer = read_buffer.lines().next().unwrap(); //read buffer gets lined in stored
                let parts: Vec<&str> = lining_buffer.split(' ').collect(); //colleced in the 'parts' vector and spilit based on spaces then stored

                println!("[ LOG ]: read_buffer\r\n{}", read_buffer);
                println!("[ LOG ]: lining_buffer\r\n{}", lining_buffer);
                println!("[ LOG ]: parts\r\n{} {} {}", parts[0], parts[1], parts[2]);

                if parts[1] == "/" {
                    stream
                        .write_all("HTTP/1.1 200 OK\r\n\r\n".as_bytes())
                        .unwrap();
                } else if parts[1].starts_with("/echo/") {
                    let echo_str = parts[1].strip_prefix("/echo/").unwrap();
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                        echo_str.len(),
                        echo_str
                    );
                    println!("[ LOG ]: echo\r\n{}", echo_str);
                    stream.write_all(response.as_bytes()).unwrap();
                } else if parts[1] == "/user-agent" {
                    let mut user_agent_str = String::from("");
                    for line in read_buffer.lines() {
                        if line.starts_with("User-Agent: ") {
                            user_agent_str = line.strip_prefix("User-Agent: ").unwrap().to_string();
                        }
                    }
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                        user_agent_str.len(),
                        user_agent_str
                    );
                    println!("[ LOG ]: user-agent\r\n{}", user_agent_str);
                    stream.write_all(response.as_bytes()).unwrap();
                } else {
                    stream
                        .write_all("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes())
                        .unwrap();
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
