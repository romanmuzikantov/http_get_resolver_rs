extern crate tokio;

use tokio::prelude::*;
use tokio::io::write_all;
use tokio::io::read_to_end;
use tokio::net::TcpStream;
use std::net::ToSocketAddrs;
use tokio::fs::File;

fn download(host: String, path: String, file_name: String) -> impl Future<Item = (), Error = ()> {
    let mut addr_iter = host.to_socket_addrs().unwrap();
    let addr = match addr_iter.next() {
            None => panic!("DNS not resolved"),
            Some(addr) => addr
    };

    let req_body = format!("GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n", path, host);

    TcpStream::connect(&addr)
        .and_then(|stream| {
            println!("Got a stream: {:?}", stream);
            write_all(stream, req_body)
        })
        .and_then(|(stream, _)| {
            println!("Got a response: {:?}", stream);
            let vec = vec![];
            read_to_end(stream, vec)
        })
        .and_then(|(_, resp)| {
            File::create(file_name).map(move |mut file| {
                println!("Writing into file: {:?}", file);
                file.write_all(&resp).unwrap();
            })
        })
        .map_err(|err| {
            eprintln!("Error: {:?}", err);
        })
}

fn main() {
    tokio::run(future::poll_fn(|| {
        let mut args = std::env::args().skip(1);

        loop {
            match (args.next(), args.next(), args.next()) {
                (Some(arg_f), Some(arg_s), Some(arg_t)) => {
                    tokio::spawn(download(arg_f, arg_s, arg_t));
                },
                _ => return Ok(Async::Ready(()))
            }
        }
    }))
}
