use netstat2::{get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo};
use crate::{Protocol, SocketState, SocketInfo, ProcessInfo};
use crate::process::ProcessMapper;

pub fn get_tcp_udp_sockets(tcp: bool, udp: bool, process: bool, mapper: &mut ProcessMapper) -> Result<Vec<SocketInfo>, String> {
    let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let mut proto_flags = ProtocolFlags::empty();
    if tcp { proto_flags |= ProtocolFlags::TCP; }
    if udp { proto_flags |= ProtocolFlags::UDP; }

    if proto_flags.is_empty() {
        return Ok(Vec::new());
    }

    let sockets = get_sockets_info(af_flags, proto_flags).map_err(|e| e.to_string())?;
    let mut result = Vec::new();

    for socket in sockets {
        let associated_pid = if process && !socket.associated_pids.is_empty() {
            Some(socket.associated_pids[0])
        } else {
            None
        };

        let process_info = associated_pid.map(|pid| {
            let name = mapper.get_process_name(pid).unwrap_or_else(|| String::from(""));
            ProcessInfo { pid, name }
        });

        match socket.protocol_socket_info {
            ProtocolSocketInfo::Tcp(tcp_info) => {
                let state = match tcp_info.state {
                    netstat2::TcpState::Established => SocketState::Established,
                    netstat2::TcpState::SynSent => SocketState::SynSent,
                    netstat2::TcpState::SynReceived => SocketState::SynRecv,
                    netstat2::TcpState::FinWait1 => SocketState::FinWait1,
                    netstat2::TcpState::FinWait2 => SocketState::FinWait2,
                    netstat2::TcpState::TimeWait => SocketState::TimeWait,
                    netstat2::TcpState::Closed => SocketState::Closed,
                    netstat2::TcpState::CloseWait => SocketState::CloseWait,
                    netstat2::TcpState::LastAck => SocketState::LastAck,
                    netstat2::TcpState::Listen => SocketState::Listen,
                    netstat2::TcpState::Closing => SocketState::Closing,
                    netstat2::TcpState::DeleteTcb => SocketState::Unknown,
                    netstat2::TcpState::Unknown => SocketState::Unknown,
                };

                result.push(SocketInfo {
                    protocol: Protocol::Tcp,
                    state,
                    recv_q: 0, // netstat2 doesn't provide recv/send queue sizes
                    send_q: 0,
                    local_addr: tcp_info.local_addr.to_string(),
                    local_port: tcp_info.local_port,
                    remote_addr: tcp_info.remote_addr.to_string(),
                    remote_port: tcp_info.remote_port,
                    process: process_info,
                });
            }
            ProtocolSocketInfo::Udp(udp_info) => {
                result.push(SocketInfo {
                    protocol: Protocol::Udp,
                    state: SocketState::Closed, // UDP is connectionless
                    recv_q: 0,
                    send_q: 0,
                    local_addr: udp_info.local_addr.to_string(),
                    local_port: udp_info.local_port,
                    remote_addr: String::from("*"),
                    remote_port: 0,
                    process: process_info,
                });
            }
        }
    }

    Ok(result)
}
