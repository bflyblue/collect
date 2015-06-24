use std::fs::OpenOptions;
use std::io::{Result, Cursor, copy};
use std::net::{UdpSocket, SocketAddr};
use std::sync::mpsc::{sync_channel, SyncSender, Receiver};
use std::thread;

struct NetData { src: SocketAddr, data: Vec<u8> }

fn network(tx: SyncSender<NetData>) -> Result<u64> {
    let socket = try!(UdpSocket::bind("0.0.0.0:9000"));

    let mut buf = [0; 65536];
    let (amt, src) = try!(socket.recv_from(&mut buf));
    let data = (&buf[..amt]).to_vec();

    tx.send(NetData {src: src, data: data});

    drop(socket);

    return Ok(0);
}

fn persist (rx: Receiver<NetData>) -> Result<()> {
    let mut f = try!(OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open("collect.raw"));

    let netdata = rx.recv().unwrap();

    println!("Received {} bytes from {}", netdata.data.len(), netdata.src);
    copy(&mut Cursor::new(netdata.data), &mut f);

    drop(f);

    return Ok(());
}

fn main() {
    let (tx, rx) = sync_channel(1000);

    let listener = thread::spawn(move|| network(tx));
    let writer   = thread::spawn(move|| persist(rx));

    listener.join();
    writer.join();
}
