use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::time::timeout;

/// Attempts to open a TCP connection to the given host and port within a 200 ms deadline,
/// reporting success back to the caller via an async channel.
///
/// # Parameters
/// * `tx: mpsc::Sender<SocketAddr>` — an async channel sender. If the connection succeeds,
///   the probed `ip` and `port` is sent through this channel.
/// * `target: SocketAddr` — target IP address and TCP port (e.g. `"192.168.1.1:1024"`).
///
/// # Behavior
/// Attempts `TcpStream::connect` to each address, but aborts the entire operation if no connection completes within 200 ms.  
/// If **any** connect attempt succeeds before the timeout, sends `SocketAddr` over `tx`.  
/// If all attempts fail or the 200 ms window elapses, does nothing (no error is propagated).
/// 
/// # Returns
/// This function returns `()`; on a successful connect, it forwards the open `ip` and `port` on `tx`.
/// In case of a timeout, DNS failure, or connection error, it silently returns without sending. 
pub async fn scan_port(tx: mpsc::Sender<SocketAddr>, target: SocketAddr) {
    if timeout(Duration::from_millis(200), TcpStream::connect(target)).await.is_ok() {
        let _ = tx.send(target).await;
    }
}
