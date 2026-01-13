use std::process::exit;
use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::time::{sleep, timeout};

const DISCOVERY_PORT: u16 = 18875; // Must match the Server's Port

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("-------------------------------------------------");
    println!("📡 Aaxion Finder: Actively Listening for Server...");
    println!("👉 Target Port: {}", DISCOVERY_PORT);
    println!("-------------------------------------------------");

    // 1. Bind to a random port to send/receive
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    socket.set_broadcast(true)?;

    let broadcast_addr = format!("255.255.255.255:{}", DISCOVERY_PORT);
    let discovery_msg = "DISCOVER_MAIN_SERVER";
    let mut buf = [0u8; 1024];

    // 2. Continuous Loop (The "Active" part)
    loop {
        // Step A: Shout "Are you there?"
        // We ignore send errors (e.g., network down) and just retry
        let _ = socket
            .send_to(discovery_msg.as_bytes(), &broadcast_addr)
            .await;

        // Step B: Listen for a reply (Wait 2 seconds max)
        let result = timeout(Duration::from_secs(2), socket.recv_from(&mut buf)).await;

        match result {
            Ok(Ok((size, peer))) => {
                // SERVER FOUND
                let response = String::from_utf8_lossy(&buf[..size]);

                // Clear screen (optional, creates a dashboard feel)
                // print!("\x1B[2J\x1B[1;1H");

                println!("✅ SERVER ONLINE!");
                println!("   📍 IP:       {}", peer.ip());
                println!("   🔗 Address:  http://{}:{}", peer.ip(), DISCOVERY_PORT);
                println!("   💬 Message:  {}", response);

                // Wait longer (5s) before checking again so we don't spam the console
                // showing that we are still connected.
                exit(0)
            }
            Ok(Err(_)) => {
                // Socket Error
                eprintln!("⚠️ Network Error. Retrying...");
                sleep(Duration::from_secs(1)).await;
            }
            Err(_) => {
                // Timeout (Server not found)
                println!("⏳ Searching... (No response on LAN)");
                // Wait 1s before shouting again
                sleep(Duration::from_secs(1)).await;
            }
        }
    }
}
