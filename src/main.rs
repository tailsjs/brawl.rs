use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};

mod bytestream;
use bytestream::ByteStream;

mod buffer;

struct PacketHeader {
    id: u16,
    length: u16,
    version: u16
}

fn handle_client(mut stream: TcpStream) {
    let mut data: [u8; 1024] = [0 as u8; 1024];
    
    while match stream.read(&mut data) {
        Ok(size) => {
            let buffer = &data;

            let header: PacketHeader = PacketHeader {
                id: buffer::read_uint16_be(buffer, 0),
                length: buffer::read_uint_be(buffer, 2, 3),
                version: buffer::read_uint16_be(buffer, 5)
            };

            let bytes: &[u8] = &buffer[7..];

            let mut bytestream = ByteStream::new(bytes);   

            println!("VInt: {}", bytestream.read_vint());
            println!("Int: {}", bytestream.read_int()); 
            println!("Bool: {}", bytestream.read_boolean()); // лан
            println!("String: {}", bytestream.read_string());
            println!("VInt: {}", bytestream.read_vint());

            println!("Received: {}, length: {}, version: {}", header.id, header.length, header.version);
            stream.write(&data[0..size]).unwrap();
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    let port: i16 = 9339;

    let listener = TcpListener::bind(format!("127.0.0.1:{port}")).unwrap();
    println!("Server listening on port {port}");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    drop(listener);
}