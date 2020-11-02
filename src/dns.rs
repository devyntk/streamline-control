// Code adapted from https://gitlab.com/open-sourceress/findme
use std::{
    collections::HashSet,
    io,
    net::{Ipv4Addr, SocketAddr, UdpSocket},
    thread::sleep,
    time::Duration,
};

/// Find devices on a local network
pub struct Opt {
    /// UDP multicast group to join/broadcast to.
    // 239.235.*.* is available for organization-local use, 70.77 is "FM" in ASCII (arbitrary)
    // https://www.iana.org/assignments/multicast-addresses/multicast-addresses.xhtml#table-multicast-addresses-12
    // default 239.255.70.77
    pub(crate) multicast_group: Ipv4Addr,

    /// Interface to bind to for sending/receiving.
    //default: 0.0.0.0
    pub(crate) host: Ipv4Addr,

    /// Port to send/receive on.
    // >=49152 is available, 50765 is arbitrary
    // default 50765
    pub(crate) port: u16,

    pub(crate) command: Command,
}

pub enum Command {
    /// Continually broadcast this device's existence for other devices to find.
    // #[structopt(name = "broadcast")]
    Broadcast {
        /// The name to broadcast as. Defaults to device's hostname.
        name: Option<String>,
    },
    /// Listen for a particular device and display its IP address.
    // #[structopt(name = "find")]
    Find {
        /// The name to search for.
        name: String,
    },
    /// Listen for all devices broadcasting, and display their names and IP addresses.
    // #[structopt(name = "list")]
    List {
        /// Show each name once, rather than every time it is received.
        // #[structopt(long = "once")]
        show_once: bool,
    }
}

pub fn run(args: Opt) -> io::Result<()> {
    let multicast_sockaddr = (args.multicast_group, args.port);
    match args.command {
        Command::Broadcast { name } => {
            let message = format!("findme:name=streamline-control");
            let sock = UdpSocket::bind((args.host, 0))?;
            loop {
                sock.send_to(message.as_bytes(), multicast_sockaddr)?;
                sleep(Duration::from_secs(1));
            }
        }
        Command::Find { name } => {
            let sock = UdpSocket::bind((args.host, args.port))?;
            sock.join_multicast_v4(&args.multicast_group, &args.host)?; // TODO IPv6
            let found_addr = {
                let expected_message = format!("findme:name={}", name);
                let mut buf = vec![0; 1024];
                loop {
                    let addr = read_from(&sock, &mut buf)?;
                    match String::from_utf8(buf) {
                        Ok(ref message) if message == &expected_message => break addr,
                        Ok(message) => buf = message.into_bytes(),
                        Err(e) => buf = e.into_bytes(),
                    }
                }
            };
            println!("{}", found_addr.ip());
        },
        Command::List { show_once } => {
            let sock = UdpSocket::bind((args.host, args.port))?;
            sock.join_multicast_v4(&args.multicast_group, &args.host)?; // TODO IPv6
            let mut seen = if show_once { Some(HashSet::new()) } else { None };
            let mut buf = vec![0; 1024];
            loop {
                let addr = read_from(&sock, &mut buf)?.ip();
                let message = match std::str::from_utf8(&buf) {
                    Ok(s) => s,
                    Err(_) => continue,
                };
                if !message.starts_with("findme:name=") { continue; }
                let name = &message[12..];
                // Insert name if we care about deduping, otherwise just print
                let print = match seen {
                    None => true,
                    Some(ref set) if set.contains(name) => false,
                    Some(ref mut set) => set.insert(name.to_owned()),
                };
                if print {
                    println!("{} {}", addr, name);
                }
            }
        },
    };
    Ok(())
}

// Part of the error message returned from peek_from/recv_from on Windows when the buffer is too small for the datagram
const TOO_SMALL_ERRMSG: &str = "the buffer used to receive a datagram into was smaller than the datagram itself";

// Read a datagram into buf without truncating, set buf's len to the size of the dgram, and return the sender's address
fn read_from(sock: &UdpSocket, buf: &mut Vec<u8>) -> io::Result<SocketAddr> {
    loop {
        // Windows: if buf is too small, returns Err, kind=Other,
        // message="A message sent on a datagram socket was larger than the internal message buffer..."
        // Linux: if buf is too small, writes what it can and returns Ok(..) normally
        match sock.peek_from(buf) {
            Ok((written, peer_addr)) if written < buf.len() => { // guard: ensure we didn't truncate (Linux)
                sock.recv_from(buf)?; // Remove packet from queue
                buf.truncate(written);
                return Ok(peer_addr);
            },
            // Re-raise errors, except for ones caused by needing to resize on Windows
            Err(e) if e.kind() != io::ErrorKind::Other || !e.to_string().contains(TOO_SMALL_ERRMSG) => Err(e)?,
            _ => {},
        }
        // If we have the entire datagram or an error, we've already returned. We need to retry with a bigger buffer.
        buf.resize(buf.len() * 2, 0);
    }
}
