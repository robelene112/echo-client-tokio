use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

const ECHO_SERVER_ADDRESS: &str = "localhost:1234";

#[tokio::main]
async fn main() {
    // connect to server
    println!("connecting to {}", ECHO_SERVER_ADDRESS);
    if let Ok(mut stream) = TcpStream::connect(ECHO_SERVER_ADDRESS).await {
        println!(
            "connected to echo server {}:{}",
            stream.local_addr().unwrap().ip(),
            stream.local_addr().unwrap().port()
        );

        // print message
        let message = "Hello, world!";
        let _ = stream.write_all(message.as_bytes()).await;
        println!("sent: {}", message);

        // read the result
        let mut buffer = [0; 1024];
        let len = stream.read(&mut buffer).await.unwrap();
        let recieved_message = String::from_utf8_lossy(&buffer);
        println!("recieved message reads: {recieved_message}, length of bytes returned = {len}");
    } else {
        println!("failed to connect to echo server {}", ECHO_SERVER_ADDRESS)
    }
}
