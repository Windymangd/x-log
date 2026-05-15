use tokio::net::{TcpListener, TcpStream};
use tokio::io::{ AsyncReadExt, AsyncWriteExt};
use bore_cli::client::Client;
use std::net::SocketAddr;

const L_PORT: u16 = 8080;
const OUT_PORT: u16 = 51515; 
const B_SERVER: &str  = "bore.pub";

#[tokio::main]
async fn main() {
    tokio::spawn(
        bore(),
        );
    server().await;
}

async fn server() {
    let l = TcpListener::bind(("127.0.0.1", L_PORT)).await.unwrap(); // the listener

    println!("listening on port {L_PORT}");

    loop {
        let (stream, addr) = l.accept().await.unwrap(); // loop for the listener
        println!("connection from {addr}");
        tokio::spawn(async move {
            handle(stream, addr).await;
        });
    }
}

async fn handle(mut stream: TcpStream, addr: SocketAddr) {
    loop {
        let mut buf = [0_u8; 1024];
        let n = stream.read(&mut buf).await.unwrap();
        let msg = String::from_utf8_lossy(&buf[..n]);
        if n == 0 {
            println!("client disconnected from {addr}");
            break;
        }
        println!("received: {msg} from {addr}");
        stream.write_all(b"ACK\n").await.unwrap();
    }
}

async fn get_and_send() {
    println!("up");
}

async fn bore() {
    // first time using bore in my own code
    let c = Client::new("127.0.0.1", L_PORT, B_SERVER, OUT_PORT, None).await.expect("failed to open tunnel");

    println!("tunnel open to {}:{}", B_SERVER, c.remote_port()); // i might change bore.pub to a custom domain/ip for the future
    
    c.listen().await.expect("tunnel error");


}
