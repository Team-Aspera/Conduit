use anyhow::Result;
use pnet::datalink;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::process::Command;
use std::sync::Arc;
use tokio::io::copy_bidirectional;
use tokio::net::{TcpListener, TcpStream, UdpSocket};
use tokio::sync::{Mutex, watch};
use tokio::time::{Duration, Instant};

type ClientMap = Arc<Mutex<HashMap<SocketAddr, (Arc<UdpSocket>, Instant)>>>;

#[derive(Debug, Clone)]
pub struct SystemReport {
    pub ip_forward_enabled: bool,
    pub nat_masquerade: Vec<String>,
    pub port_forwards: Vec<String>,
    pub listening_ports: Vec<String>,
    pub active_connections: Vec<String>,
    pub iptables_failed: bool,
}

pub struct InterfaceInfo {
    pub name: String,
}

pub fn get_interfaces() -> Vec<InterfaceInfo> {
    datalink::interfaces()
        .into_iter()
        .map(|i| InterfaceInfo { name: i.name })
        .collect()
}

pub fn get_system_network_report() -> SystemReport {
    let mut report = SystemReport {
        ip_forward_enabled: false,
        nat_masquerade: vec![],
        port_forwards: vec![],
        listening_ports: vec![],
        active_connections: vec![],
        iptables_failed: false,
    };

    report.ip_forward_enabled = std::fs::read_to_string("/proc/sys/net/ipv4/ip_forward")
        .unwrap_or_default()
        .trim()
        == "1";

    if let Ok(output) = Command::new("iptables").args(["-t", "nat", "-S"]).output() {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                if line.contains("MASQUERADE") {
                    report.nat_masquerade.push(line.to_string());
                } else if line.contains("DNAT") || line.contains("REDIRECT") {
                    report.port_forwards.push(line.to_string());
                }
            }
        } else {
            report.iptables_failed = true;
        }
    } else {
        report.iptables_failed = true;
    }

    if let Ok(output) = Command::new("ss").args(["-tlnpu"]).output()
        && output.status.success()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines().skip(1) {
            report.listening_ports.push(line.to_string());
        }
    }

    if let Ok(output) = Command::new("ss").args(["-apn"]).output()
        && output.status.success()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            if line.contains("conduit")
                && (line.contains("ESTAB") || line.contains("LISTEN") || line.contains("UNCONN"))
            {
                report.active_connections.push(line.to_string());
            }
        }
    }

    report
}

pub fn detect_system_forward_status() -> (bool, Vec<String>, bool) {
    let report = get_system_network_report();
    let mut active_wans = Vec::new();
    for rule in &report.nat_masquerade {
        let parts: Vec<&str> = rule.split_whitespace().collect();
        if let Some(pos) = parts.iter().position(|&r| r == "-o")
            && let Some(iface) = parts.get(pos + 1)
        {
            active_wans.push(iface.to_string());
        }
    }
    let active = report.ip_forward_enabled && !active_wans.is_empty();
    (active, active_wans, report.iptables_failed)
}

// --- 系统转发控制 ---

pub fn start_system_forwarding(
    lan_shares: &[(String, String, String, Vec<String>)],
) -> std::io::Result<()> {
    let mut commands = Vec::new();
    commands.push("echo 1 > /proc/sys/net/ipv4/ip_forward".to_string());
    for (lan_if, host_ip, mask, wan_ifs) in lan_shares {
        commands.push(format!(
            "ip addr add {}/{} dev {} 2>/dev/null || true",
            host_ip, mask, lan_if
        ));
        commands.push(format!("ip link set {} up", lan_if));
        for wan_if in wan_ifs {
            commands.push(format!(
                "iptables -t nat -D POSTROUTING -o {} -j MASQUERADE 2>/dev/null || true",
                wan_if
            ));
            commands.push(format!(
                "iptables -t nat -A POSTROUTING -o {} -j MASQUERADE",
                wan_if
            ));
            commands.push(format!("iptables -D FORWARD -i {} -o {} -m state --state RELATED,ESTABLISHED -j ACCEPT 2>/dev/null || true", wan_if, lan_if));
            commands.push(format!(
                "iptables -A FORWARD -i {} -o {} -m state --state RELATED,ESTABLISHED -j ACCEPT",
                wan_if, lan_if
            ));
            commands.push(format!(
                "iptables -D FORWARD -i {} -o {} -j ACCEPT 2>/dev/null || true",
                lan_if, wan_if
            ));
            commands.push(format!(
                "iptables -A FORWARD -i {} -o {} -j ACCEPT",
                lan_if, wan_if
            ));
        }
    }
    run_batch_as_root(commands)
}

pub fn stop_system_forwarding(
    lan_shares: &[(String, String, String, Vec<String>)],
) -> std::io::Result<()> {
    let mut commands = Vec::new();
    for (lan_if, host_ip, mask, wan_ifs) in lan_shares {
        for wan_if in wan_ifs {
            commands.push(format!(
                "iptables -t nat -D POSTROUTING -o {} -j MASQUERADE 2>/dev/null || true",
                wan_if
            ));
            commands.push(format!("iptables -D FORWARD -i {} -o {} -m state --state RELATED,ESTABLISHED -j ACCEPT 2>/dev/null || true", wan_if, lan_if));
            commands.push(format!(
                "iptables -D FORWARD -i {} -o {} -j ACCEPT 2>/dev/null || true",
                lan_if, wan_if
            ));
        }
        commands.push(format!(
            "ip addr del {}/{} dev {} 2>/dev/null || true",
            host_ip, mask, lan_if
        ));
    }
    commands.push("echo 0 > /proc/sys/net/ipv4/ip_forward".to_string());
    run_batch_as_root(commands)
}

fn run_batch_as_root(commands: Vec<String>) -> std::io::Result<()> {
    if commands.is_empty() {
        return Ok(());
    }
    let full_script = commands.join(" && ");
    let status = Command::new("pkexec")
        .arg("sh")
        .arg("-c")
        .arg(full_script)
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err(std::io::Error::other("Root failed"))
    }
}

// --- TCP 转发 ---

pub async fn start_tcp_forward(
    src_addr: String,
    src_port: u16,
    dst_addr: String,
    dst_port: u16,
    mut stop_rx: watch::Receiver<bool>,
) -> Result<()> {
    let src_socket = format!("{}:{}", src_addr, src_port);
    let dst_socket = format!("{}:{}", dst_addr, dst_port);
    let listener = TcpListener::bind(&src_socket).await?;

    loop {
        tokio::select! {
            accept_res = listener.accept() => {
                if let Ok((mut client, _)) = accept_res {
                    let d = dst_socket.clone();
                    let mut stop_rx_clone = stop_rx.clone();
                    tokio::spawn(async move {
                        if let Ok(mut server) = TcpStream::connect(&d).await {
                            tokio::select! {
                                _ = copy_bidirectional(&mut client, &mut server) => {},
                                _ = stop_rx_clone.changed() => {},
                            }
                        }
                    });
                }
            }
            _ = stop_rx.changed() => {
                if *stop_rx.borrow() { break; }
            }
        }
    }
    Ok(())
}

// --- UDP 转发 ---

pub async fn start_udp_forward(
    src_addr: String,
    src_port: u16,
    dst_addr: String,
    dst_port: u16,
    mut stop_rx: watch::Receiver<bool>,
) -> Result<()> {
    let src_socket_addr = format!("{}:{}", src_addr, src_port);
    let dst_socket_addr = format!("{}:{}", dst_addr, dst_port);
    let socket = Arc::new(UdpSocket::bind(&src_socket_addr).await?);

    let clients: ClientMap = Arc::new(Mutex::new(HashMap::new()));
    let mut buf = [0u8; 4096];

    loop {
        tokio::select! {
            res = socket.recv_from(&mut buf) => {
                if let Ok((len, addr)) = res {
                    let mut guard = clients.lock().await;
                    let target = if let Some(c) = guard.get_mut(&addr) {
                        c.1 = Instant::now();
                        c.0.clone()
                    } else {
                        let t = Arc::new(UdpSocket::bind("0.0.0.0:0").await?);
                        t.connect(&dst_socket_addr).await?;

                        let s_clone = socket.clone();
                        let t_clone = t.clone();
                        let mut stop_rx_clone = stop_rx.clone();

                        tokio::spawn(async move {
                            let mut b = [0u8; 4096];
                            loop {
                                tokio::select! {
                                    n_res = t_clone.recv(&mut b) => {
                                        if let Ok(n) = n_res {
                                            let _ = s_clone.send_to(&b[..n], addr).await;
                                        } else { break; }
                                    }
                                    _ = stop_rx_clone.changed() => { break; }
                                }
                            }
                        });
                        guard.insert(addr, (t.clone(), Instant::now()));
                        t
                    };
                    let _ = target.send(&buf[..len]).await;
                }
            }
            _ = stop_rx.changed() => {
                if *stop_rx.borrow() { break; }
            }
            _ = tokio::time::sleep(Duration::from_secs(30)) => {
                let mut guard = clients.lock().await;
                guard.retain(|_, (_, t)| t.elapsed() < Duration::from_secs(60));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    #[tokio::test]
    async fn test_tcp_forward_proxies_data() {
        let echo_listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let echo_port = echo_listener.local_addr().unwrap().port();
        tokio::spawn(async move {
            loop {
                if let Ok((mut stream, _)) = echo_listener.accept().await {
                    tokio::spawn(async move {
                        let mut buf = vec![0u8; 4096];
                        loop {
                            if let Ok(n) =
                                tokio::io::AsyncReadExt::read(&mut stream, &mut buf).await
                            {
                                if n == 0 {
                                    break;
                                }
                                let _ = tokio::io::AsyncWriteExt::write_all(&mut stream, &buf[..n])
                                    .await;
                            } else {
                                break;
                            }
                        }
                    });
                }
            }
        });

        let (stop_tx, stop_rx) = watch::channel(false);
        let forwarder_port = {
            let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
            let port = listener.local_addr().unwrap().port();
            drop(listener);
            port
        };

        let fwd = tokio::spawn(async move {
            start_tcp_forward(
                "127.0.0.1".into(),
                forwarder_port,
                "127.0.0.1".into(),
                echo_port,
                stop_rx,
            )
            .await
        });

        tokio::time::sleep(Duration::from_millis(100)).await;

        let mut client = TcpStream::connect(format!("127.0.0.1:{}", forwarder_port))
            .await
            .unwrap();
        let msg = b"hello conduit";
        client.write_all(msg).await.unwrap();

        let mut buf = vec![0u8; 128];
        let n = client.read(&mut buf).await.unwrap();
        assert_eq!(&buf[..n], msg);

        stop_tx.send(true).unwrap();
        let _ = tokio::time::timeout(Duration::from_secs(2), fwd).await;
    }

    #[tokio::test]
    async fn test_udp_forward_proxies_data() {
        let echo = UdpSocket::bind("127.0.0.1:0").await.unwrap();
        let echo_port = echo.local_addr().unwrap().port();
        tokio::spawn(async move {
            let mut buf = vec![0u8; 4096];
            loop {
                if let Ok((n, addr)) = echo.recv_from(&mut buf).await {
                    let _ = echo.send_to(&buf[..n], addr).await;
                }
            }
        });

        let (stop_tx, stop_rx) = watch::channel(false);
        let fwd_port = {
            let s = UdpSocket::bind("127.0.0.1:0").await.unwrap();
            s.local_addr().unwrap().port()
        };

        let fwd = tokio::spawn(async move {
            start_udp_forward(
                "127.0.0.1".into(),
                fwd_port,
                "127.0.0.1".into(),
                echo_port,
                stop_rx,
            )
            .await
        });

        tokio::time::sleep(Duration::from_millis(200)).await;

        let client = UdpSocket::bind("127.0.0.1:0").await.unwrap();
        client
            .connect(format!("127.0.0.1:{}", fwd_port))
            .await
            .unwrap();
        let msg = b"hello conduit udp";
        client.send(msg).await.unwrap();

        let mut buf = vec![0u8; 128];
        let n = loop {
            tokio::time::sleep(Duration::from_millis(50)).await;
            if let Ok(n) = client.recv(&mut buf).await {
                break n;
            }
        };
        assert_eq!(&buf[..n], msg);

        let _ = stop_tx.send(true);
        let _ = tokio::time::timeout(Duration::from_secs(2), fwd).await;
    }

    #[tokio::test]
    async fn test_tcp_forward_stops_on_signal() {
        let (stop_tx, stop_rx) = watch::channel(false);
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        drop(listener);

        let fwd = tokio::spawn(async move {
            start_tcp_forward("127.0.0.1".into(), port, "127.0.0.1".into(), 9999, stop_rx).await
        });

        tokio::time::sleep(Duration::from_millis(100)).await;
        let _ = TcpStream::connect(format!("127.0.0.1:{}", port)).await;

        let _ = stop_tx.send(true);
        let result = tokio::time::timeout(Duration::from_secs(3), fwd).await;
        assert!(result.is_ok(), "Forwarder did not stop in time");
        assert!(result.unwrap().is_ok());
    }
}
