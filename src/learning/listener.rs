use std::net::{Shutdown, TcpListener, TcpStream};
use std::io::Read;
use std::io::Write;
use std::thread;

pub fn main() {
  let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

  for stream in listener.incoming() {
    match stream {
      Ok(stream) => {
        thread::spawn(|| {
          handle_client(stream)
        });
      },
      Err(_e) => {}
    };
  }
}

fn handle_client(mut stream: TcpStream) {
  println!("handle_client");
  let mut data : Vec<u8> = Vec::new();
  match stream.read(&mut data) {
    Ok(val) => println!("Read {} bytes", val),
    Err(_e) => { return }
  };

  let content_to_write = "Hello world from my tcp listenter and tcp stream writer\nThis is pretty cool!\n";
  match stream.write_fmt(format_args!("Content-Length: {}\n\n{}\n", content_to_write.len(), content_to_write)) {
    Ok(_) => { println!("Write succeeded"); },
    Err(_) => { println!("Unable to write"); }
  };

  match stream.shutdown(Shutdown::Both) {
    Ok(_) => { println!("Stream shut down successfully"); },
    Err(_) => { println!("Unable to shut down stream"); }
  };
}
