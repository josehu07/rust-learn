use std::io;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::process;
use std::fs;
use std::thread;
use std::time::Duration;

use web_server::ThreadPool;


// Web server, listening on requests.
fn main() {

    // Bind a listener to localhost:7878.
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap_or_else(|err| {
        eprintln!("ERROR binding listener: {}", err);
        process::exit(1);
    });

    // Create a thread pool for responding requests in a multithreading way.
    let thread_pool = ThreadPool::new(6);

    // Listening on the first 10 incoming streams.
    println!("√ Rusty web server online!");
    for stream in listener.incoming().take(10) {
        match stream {
            Ok(stream) => thread_pool.exec(|| { respond(stream).unwrap(); }),
            Err(err) => eprintln!("ERROR incoming request: {}", err),
        }
    }
    println!("ø Rusty web server shutting down...");
}


// Respond to a stream.
fn respond(mut stream : TcpStream) -> io::Result<()> {

    // Create a buffer of 512 bytes and read the stream.
    let mut buf = [0 as u8 ; 512];
    stream.read(&mut buf)?;
    // println!("† Got request:\n{}", String::from_utf8_lossy(&buf[..]));

    // Create respond to the stream:
    //   URI = "/", sending back "hello.html".
    //   URI = "/sleep", sending back "hello.html" after sleeping for 3s.
    //   Otherwise, 404.
    let response = if buf.starts_with(b"GET / HTTP/1.1\r\n") {
        format!("HTTP/1.1 200 OK\r\n\r\n{}", fs::read_to_string("hello.html")?)
    } else if buf.starts_with(b"GET /sleep HTTP/1.1\r\n") {
        thread::sleep(Duration::from_secs(3));
        format!("HTTP/1.1 200 OK\r\n\r\n{}", fs::read_to_string("hello.html")?)
    } else {
        format!("HTTP/1.1 404 NOT FOUND\r\n\r\n{}", fs::read_to_string("404.html")?)
    };

    // Write the resopnd back.
    stream.write(response.as_bytes())?;
    stream.flush()?;
    println!("† Responded.");

    Ok(())
}
