use clap::Parser;
use std::fmt;

/// A macOS clone of the Ubuntu `ss` (socket statistics) utility.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Display TCP sockets
    #[arg(short = 't', long)]
    pub tcp: bool,

    /// Display UDP sockets
    #[arg(short = 'u', long)]
    pub udp: bool,

    /// Display UNIX domain sockets
    #[arg(short = 'x', long)]
    pub unix: bool,

    /// Display both listening and non-listening (for TCP) sockets
    #[arg(short = 'a', long)]
    pub all: bool,

    /// Display only listening sockets
    #[arg(short = 'l', long)]
    pub listening: bool,

    /// Show process using socket
    #[arg(short = 'p', long)]
    pub process: bool,

    /// Do not resolve service names / IP addresses (numeric output)
    #[arg(short = 'n', long)]
    pub numeric: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Protocol {
    Tcp,
    Udp,
    Unix,
}

impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Protocol::Tcp => write!(f, "tcp"),
            Protocol::Udp => write!(f, "udp"),
            Protocol::Unix => write!(f, "u_str"), // or u_dgr for datagram
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SocketState {
    Established,
    SynSent,
    SynRecv,
    FinWait1,
    FinWait2,
    TimeWait,
    Closed,
    CloseWait,
    LastAck,
    Listen,
    Closing,
    Unknown,
}

impl fmt::Display for SocketState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            SocketState::Established => "ESTAB",
            SocketState::SynSent => "SYN-SENT",
            SocketState::SynRecv => "SYN-RECV",
            SocketState::FinWait1 => "FIN-WAIT-1",
            SocketState::FinWait2 => "FIN-WAIT-2",
            SocketState::TimeWait => "TIME-WAIT",
            SocketState::Closed => "UNCONN",
            SocketState::CloseWait => "CLOSE-WAIT",
            SocketState::LastAck => "LAST-ACK",
            SocketState::Listen => "LISTEN",
            SocketState::Closing => "CLOSING",
            SocketState::Unknown => "UNKNOWN",
        };
        write!(f, "{}", s)
    }
}

pub mod process;
pub mod net;
pub mod unix;

#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct SocketInfo {
    pub protocol: Protocol,
    pub state: SocketState,
    pub recv_q: u32,
    pub send_q: u32,
    pub local_addr: String,
    pub local_port: u16,
    pub remote_addr: String,
    pub remote_port: u16,
    pub process: Option<ProcessInfo>,
}

fn format_address(addr: &str, port: u16) -> String {
    if addr.contains(':') {
        format!("[{}]:{}", addr, port)
    } else {
        format!("{}:{}", addr, port)
    }
}

fn main() {
    let mut cli = Cli::parse();
    
    // If no protocol is specified, default to all like ss does
    if !cli.tcp && !cli.udp && !cli.unix {
        cli.tcp = true;
        cli.udp = true;
        cli.unix = true;
    }

    let mut mapper = process::ProcessMapper::new();
    let mut sockets = Vec::new();

    if cli.tcp || cli.udp {
        match net::get_tcp_udp_sockets(cli.tcp, cli.udp, cli.process, &mut mapper) {
            Ok(mut socks) => sockets.append(&mut socks),
            Err(e) => eprintln!("Error retrieving tcp/udp sockets: {}", e),
        }
    }

    if cli.unix {
        match unix::get_unix_sockets(cli.process) {
            Ok(mut socks) => sockets.append(&mut socks),
            Err(e) => eprintln!("Error retrieving unix sockets: {}", e),
        }
    }

    println!("{:<10} {:<8} {:<8} {:<40} {:<40} {}", "State", "Recv-Q", "Send-Q", "Local Address:Port", "Peer Address:Port", "Process");
    
    for sock in sockets {
        // Filter listening
        if cli.listening && sock.state != SocketState::Listen {
            continue;
        }
        // Filter not-listening if 'all' is not provided and not strictly 'listening'
        if !cli.all && !cli.listening && sock.state == SocketState::Listen {
            continue;
        }

        let local = if sock.protocol == Protocol::Unix {
            sock.local_addr.clone()
        } else {
            format_address(&sock.local_addr, sock.local_port)
        };
        
        let remote = if sock.protocol == Protocol::Unix {
            sock.remote_addr.clone()
        } else if sock.remote_addr == "*" {
            "*.*".to_string()
        } else {
            format_address(&sock.remote_addr, sock.remote_port)
        };

        let proc_str = if let Some(p) = sock.process {
            format!("users:((\"{}\",pid={}))", p.name, p.pid)
        } else {
            "".to_string()
        };

        println!("{:<10} {:<8} {:<8} {:<40} {:<40} {}", 
            sock.state.to_string(), 
            sock.recv_q, 
            sock.send_q, 
            local, 
            remote, 
            proc_str
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parsing() {
        let args = vec!["ss-mac", "-t", "-l", "-n", "-p"];
        let cli = Cli::parse_from(args);
        assert!(cli.tcp);
        assert!(!cli.udp);
        assert!(cli.listening);
        assert!(cli.numeric);
        assert!(cli.process);
    }
}
