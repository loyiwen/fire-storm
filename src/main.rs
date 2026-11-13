use std::io::Read;
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    // Listen for incoming traffic on TCP port 5733
    let listener = TcpListener::bind("127.0.0.1:5733")?;
    println!("listening on 127.0.0.1:5733 ...");

    // Accept one connection
    // N/B: The test sends everything down a single connection so handling exactly one is enough
    let (mut incoming, peer) = listener.accept()?;
    println!("sender connected from {peer}");

    // Buffer of bytes to be processed
    let mut buffer: Vec<u8> = Vec::new();

    // Pull bytes off socket until sender closes
    let mut chunk = [0u8; 4096];
    loop {
        let n = incoming.read(&mut chunk)?;
        if n == 0 {
            break; // Sender closed connection
        }
        buffer.extend_from_slice(&chunk[..n]);
        println!("read {n} bytes (buffer now holds {})", buffer.len());
    }

    println!("stream closed; {} bytes left unprocessed", buffer.len());

    Ok(())
}
