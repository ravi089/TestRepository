extern crate tokio;
extern crate tokio_codec;
extern crate bytes;
extern crate tokio_io;
extern crate tokio_stdin_stdout;

use std::{io, str, env, thread};
use std::time::Duration;
use bytes::*;
use tokio::io::stdin;
use tokio_io::codec::*;
use tokio::fs::File;
use tokio::codec::Decoder;
use tokio::net::TcpListener;
use tokio::prelude::*;
use tokio_codec::{Framed, LinesCodec, BytesCodec};
use tokio_codec::{FramedRead, FramedWrite};
use std::net::SocketAddr;

fn main() {
    let addr = env::args().nth(1).unwrap_or("127.0.0.1:12345".to_string());
    let addr = addr.parse::<SocketAddr>().expect("Address parsing error");

    let stdin = tokio_stdin_stdout::stdin(0);
    //let stdout = tokio_stdin_stdout::stdout(0); // .make_sendable();

    let listener = TcpListener::bind(&addr).expect("Connection error");
    println!("Listening on: {}", addr);

    let server = listener
        .incoming()
        .map_err(|err| println!("failed to accept socket; error= {:?}", err))
        .for_each(move |stream| {
            println!("Accepted connection from: {}", stream.peer_addr().unwrap());

            let stdin = tokio_stdin_stdout::stdin(0);
            let stdout = tokio_stdin_stdout::stdout(0); // .make_sendable();

            let stdin = FramedRead::new(stdin, LinesCodec::new());
            let stdout = FramedWrite::new(stdout, LinesCodec::new());
         
            let future = stdin
                .map(move |line| {
                    println!("Sending line: {}", line);
                    line
                })
                .forward(stdout)
                .map(|_| ())
                .map_err(|err| println!("error sending data"));

            tokio::spawn(future)
        });

    tokio::run(server);
}
