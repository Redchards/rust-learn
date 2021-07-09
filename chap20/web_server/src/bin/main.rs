use std::fs;
use std::thread;
use std::time::Duration;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use web_server::ThreadPool;

fn main() -> Result<(), String> 
{
    let listener = TcpListener::bind("127.0.0.1:7878").map_err(|e| e.to_string())?;
    let mut thread_pool = ThreadPool::new(20).map_err(|e| e.to_string())?;

    for stream in listener.incoming()  
    {
        stream
            .map(|stream| spawn_connection_handler(stream, &mut thread_pool))
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

fn spawn_connection_handler(stream: TcpStream, thread_pool: &mut ThreadPool)
{
    thread_pool.execute(||
    {
        handle_connection(stream);
    });
}

fn send_response(stream: &mut TcpStream, response: &String)
{
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_connection(mut stream: TcpStream)
{
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let big_slep = b"GET /big_slep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get)
    {
        ("HTTP/1.1 200 OK", "resources/index.html")
    }
    else if buffer.starts_with(big_slep)
    {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "resources/index.html")
    }
    else
    {
        ("HTTP/1.1 404 NOT FOUND", "resources/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!
    (
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    send_response(&mut stream, &response);
}