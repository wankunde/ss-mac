use crate::{Protocol, SocketInfo, SocketState, ProcessInfo};
use std::process::Command;

pub fn get_unix_sockets(process: bool) -> Result<Vec<SocketInfo>, String> {
    let mut cmd = Command::new("lsof");
    cmd.arg("-U").arg("-F").arg("pcn");

    let output = cmd.output().map_err(|e| format!("Failed to execute lsof: {}", e))?;
    
    // Ignore error exit codes because lsof often returns 1 if it encounters permissions issues
    // but still gives useful output.
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    let mut sockets = Vec::new();
    let mut current_pid: Option<u32> = None;
    let mut current_cmd: Option<String> = None;

    for line in stdout.lines() {
        if line.is_empty() {
            continue;
        }

        let prefix = line.chars().next().unwrap();
        let value = &line[1..];

        match prefix {
            'p' => {
                current_pid = value.parse::<u32>().ok();
            }
            'c' => {
                current_cmd = Some(value.to_string());
            }
            'f' => {
                // file descriptor line, skip but expect 'n' next
            }
            'n' => {
                // Name or connected inode
                let path = value.to_string();
                
                let process_info = if process {
                    if let (Some(pid), Some(name)) = (current_pid, &current_cmd) {
                        Some(ProcessInfo {
                            pid,
                            name: name.clone(),
                        })
                    } else {
                        None
                    }
                } else {
                    None
                };

                sockets.push(SocketInfo {
                    protocol: Protocol::Unix,
                    state: SocketState::Established, // UNIX sockets from lsof are usually active/established
                    recv_q: 0,
                    send_q: 0,
                    local_addr: path,
                    local_port: 0,
                    remote_addr: String::from("*"),
                    remote_port: 0,
                    process: process_info,
                });
            }
            _ => {}
        }
    }

    Ok(sockets)
}
