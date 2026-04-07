use netstat2::{get_sockets_info, AddressFamilyFlags, ProtocolFlags};

fn main() {
    let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let proto_flags = ProtocolFlags::TCP | ProtocolFlags::UDP;

    match get_sockets_info(af_flags, proto_flags) {
        Ok(sockets) => {
            println!("Got {} sockets", sockets.len());
            for socket in sockets.into_iter().take(5) {
                println!("{:?}", socket);
            }
        }
        Err(e) => {
            eprintln!("Error getting sockets info: {}", e);
        }
    }
}
