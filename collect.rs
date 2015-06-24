use std::io;
use std::net::{UdpSocket, SocketAddr};
use std::sync::mpsc::{sync_channel, SyncSender, Receiver};
use std::thread;

struct NetData { src: SocketAddr, len: usize, data: [u8; 65536] }

fn network(tx: SyncSender<NetData>) -> io::Result<u64> {
    let socket = try!(UdpSocket::bind("0.0.0.0:9000"));

    let mut buf = [0; 65536];
    let (amt, src) = try!(socket.recv_from(&mut buf));

    tx.send(NetData {src: src, len: amt, data: buf});

    drop(socket);

    return Ok(0);
}

fn persist (rx: Receiver<NetData>) {
    let netdata = rx.recv().unwrap();

    println!("Received {} bytes from {}", netdata.len, netdata.src);
}

fn main() {
    let (tx, rx) = sync_channel(1000);

    let listener = thread::spawn(move|| network(tx));
    let writer   = thread::spawn(move|| persist(rx));

    listener.join();
    writer.join();
}
