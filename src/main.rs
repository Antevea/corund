use std::fs;
use std::error::Error;
use std::net::TcpListener;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread;

#[derive(Debug)]
enum Method {
    GET,
    PUT,
    POST,
    DELETE,
    OPTIONS,
    HEAD,
    TRACE,
    CONNECT,
    PATCH,
    UNKNOWN,
}

fn main() {
	let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

	for stream in listener.incoming() {
		let stream = stream.unwrap();
        let handle_thread = thread::spawn(|| handle_stream(stream));
        handle_thread.join().unwrap();
	}
}

fn handle_stream(mut stream: TcpStream) {
	let mut buffer = [0; 1024];
	stream.read(&mut buffer).unwrap();

    let clean_buffer: Vec<u8> = buffer.iter()
		.filter(|&x| *x != 0)
		.cloned()
		.collect::<Vec<u8>>();
	let request = String::from_utf8(clean_buffer).unwrap();
    
    let mut corund_res: Result<Method, Box<dyn Error>> = Ok(Method::UNKNOWN);

    let response_method = request.split(' ').next().unwrap();
    match response_method {
        "GET" => corund_res = handle_get(&stream),
        "PUT" => println!("INFO: Not implemented yet method: {:?}", Method::PUT),
        "POST" => corund_res = handle_post(&request),
        "HEAD" => corund_res = handle_head(&stream),
        "DELETE" => println!("INFO: Not implemented yet method: {:?}", Method::DELETE),
        "OPTIONS" => println!("INFO: Not implemented yet method: {:?}", Method::OPTIONS),
        "TRACE" => println!("INFO: Not implemented yet method: {:?}", Method::TRACE),
        "CONNECT" => println!("INFO: Not implemented yet method: {:?}", Method::CONNECT),
        "PATCH" => println!("INFO: Not implemented yet method: {:?}", Method::PATCH),
        _ => {
            println!("Unsoported method");
        },
    };
    
    if let Ok(_) = corund_res {
        println!("Error: {:?}", response_method);
    } else {
        println!("Sucess request to method: {:?}", response_method);
    }
}

fn handle_post(request_post: &str) -> Result<Method, Box<dyn Error>> {
    let post_content = request_post.rsplit("\n").next().unwrap();
    println!("post_content: {}", post_content);
    println!("post_content len: {}", post_content.len());
    Ok(Method::POST)
}

fn handle_get(mut stream: &TcpStream) -> Result<Method, Box<dyn Error>> {
	let file_path = "index.html".to_string();
	let content = fs::read_to_string(file_path.clone()).
        expect(&format!("Error: no such file or directory: {}", file_path));

	let response = format!(
		"HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
		content.len(),
		content
	);
	stream.write(response.as_bytes())?;

    stream.flush()?;
    
    Ok(Method::GET)
}

fn handle_head(mut stream: &TcpStream) -> Result<Method, Box<dyn Error>> {
    let response = format!("HTTP/1.1 200 OK\r\nContent-Length: 0\r");
    stream.write(response.as_bytes()).expect("Cant send HEAD response to client");
    Ok(Method::HEAD)
}
