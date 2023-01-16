use tokio::net::{TcpListener, TcpSocket, TcpStream};

use std::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use rand::prelude::*;


#[tokio::main]
async fn main() -> io::Result<()> {
    let address = "127.0.0.1";
    let port = "80";

    let listener = TcpListener::bind(format!("{}:{}", address, port)).await?;

    loop {
        println!("Starting to listen for message");
        let (mut socket, _) = listener.accept().await?;
        println!("Received connection");

        // let mut buf = vec![0; 1024];
        // let n = socket.read(&mut buf).await.expect("failed to read data from socket");
        // loop {
        //     socket.write_all(&buf[0..n]).await.expect("failed to write data to socket")
        // }

        tokio::spawn(async move {
            println!("Spawning");
            handle_connection(&mut socket).await;
        });
    }
}


async fn handle_connection(socket: &mut TcpStream) {
    let y: u8 = rand::thread_rng().gen();

    let mut buf = vec![0; 1024];

    println!("Before the loop");

    loop {
        println!("Reading first N");

        let n = socket.read(&mut buf).await.expect("failed to read data from socket");

        println!("{}", n);

        if n == 0 {
            println!("N was zero");
            return;
        }

        buf.insert(0,y);

        println!("{}, random is: ", y);

        loop {
            socket.write_all(&buf[0..n + 1]).await.expect("failed to write data to socket");
        }
    }
}