use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};


// Return byte offset of the head of next "CTFS" marker in `buf` (or None if there isn't one)
fn find_marker(buf: &[u8]) -> Option<usize> {
    buf.windows(4).position(|w| w == b"CTFS")
}


// Send bytes to the forward port; open connection on first use
fn forward_bytes(conn: &mut Option<TcpStream>, data: &[u8]) -> std::io::Result<()> {
    if conn.is_none() {
        *conn = Some(TcpStream::connect("127.0.0.1:5744")?);
    }
    conn.as_mut().unwrap().write_all(data)
}


fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:5733")?;
    println!("listening on 127.0.0.1:5733 ...");
    let (mut incoming, peer) = listener.accept()?;
    println!("sender connected from {peer}");
    
    let mut forward: Option<TcpStream> = None;
    let mut buffer: Vec<u8> = Vec::new();
    let mut packets: BTreeMap<u16, Vec<u8>> = BTreeMap::new();

    // Read stream until sender closes
    let mut chunk = [0u8; 4096];
    loop {
        let n = incoming.read(&mut chunk)?;
        if n == 0 {
            break; // Sender closed connection
        }

        buffer.extend_from_slice(&chunk[..n]);
        loop {
            if buffer.len() < 4 {
                break; // Not enough to check for CTFS marker; wait for more
            }

            if buffer.starts_with(b"CTFS") {
                if buffer.len() < 16 {
                    break; // Header hasn't fully arrived yet
                }
                
                let seq = u16::from_be_bytes([buffer[4], buffer[5]]);
                let length = u16::from_be_bytes([buffer[6], buffer[7]]) as usize;
                let total = 16 + length;
                if buffer.len() < total {
                    break; // Whole packet hasn't arrived yet
                }

                // Packet complete
                packets.insert(seq, buffer[16..total].to_vec());
                buffer.drain(0..total);
            } else {
                match find_marker(&buffer) {
                    Some(pos) => {
                        forward_bytes(&mut forward, &buffer[0..pos])?;
                        buffer.drain(0..pos);
                    }
                    None => {
                        let upto = buffer.len() - 3;
                        forward_bytes(&mut forward, &buffer[0..upto])?;
                        buffer.drain(0..upto);
                        break;
                    }
                }
            }
        }
    }

    // Flush any bytes held back for possible split marker
    if !buffer.is_empty() {
        forward_bytes(&mut forward, &buffer)?;
    }

    // Reassemble payloads
    let mut document: Vec<u8> = Vec::new();
    for payload in packets.values() {
        document.extend_from_slice(payload);
    }

    std::fs::write("output.pdf", &document)?;
    println!("wrote output.pdf ({} bytes)", document.len());

    Ok(())
}
