use std::collections::BTreeMap;
use std::io::Read;
use std::net::TcpListener;


// Return byte offset of the head of next "CTFS" marker in `buf` (or None if there isn't one)
fn find_marker(buf: &[u8]) -> Option<usize> {
    buf.windows(4).position(|w| w == b"CTFS")
}


fn main() -> std::io::Result<()> {
    // Listen for incoming traffic on TCP port 5733
    let listener = TcpListener::bind("127.0.0.1:5733")?;
    println!("listening on 127.0.0.1:5733 ...");

    // Accept one connection
    // N/B: The test sends everything down a single connection so handling exactly one is enough
    let (mut incoming, peer) = listener.accept()?;
    println!("sender connected from {peer}");

    let mut buffer: Vec<u8> = Vec::new();
    let mut packets: BTreeMap<u16, Vec<u8>> = BTreeMap::new();

    // Pull bytes off socket until sender closes
    let mut chunk = [0u8; 4096];
    loop {
        let n = incoming.read(&mut chunk)?;
        if n == 0 {
            break; // Sender closed connection
        }
        buffer.extend_from_slice(&chunk[..n]);
        println!("read {n} bytes (buffer now holds {})", buffer.len());

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
                println!("CTFS packet: seq={seq}, length={length}");
                packets.insert(seq, buffer[16..total].to_vec());
                buffer.drain(0..total);
            } else {
                match find_marker(&buffer) {
                    Some(pos) => {
                        buffer.drain(0..pos);
                    }
                    None => {
                        buffer.drain(0..buffer.len() - 3);
                        break;
                    }
                }
            }
        }
    }

    println!("stream closed; {} bytes left unprocessed", buffer.len());
    
    for (seq, payload) in &packets {
        println!("seq {seq}: {} bytes", payload.len());
    }
    println!("{} packets total", packets.len());

    Ok(())
}
