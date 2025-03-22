use std::io::Error;
use std::net::{IpAddr, TcpStream};
use std::time::Duration;

fn main() -> Result<(), Error> {
    let ip_v6s = get_ipv6s();
    let ports: Vec<u16> = vec![8000, 25565, 19132]; // Change this to the ports you want to check
    if ip_v6s.is_empty() {
        println!("No valid global IPv6 address found.");
    } else {
        for ip in &ip_v6s {
            println!("IPv6 Address: {}", ip);
            let ip_addr: IpAddr = ip.parse().unwrap();

            for port in &ports {
                println!("Checking port {} on {}", port, ip);
                if is_port_open(&ip_addr, *port) {
                    println!("Port {} is open on {}", port, ip);
                } else {
                    println!("Port {} is closed on {}", port, ip);
                }
            }
        }
    }
    Ok(())
}

fn get_ipv6s() -> Vec<String> {
    let mut ips = vec![];
    let interfaces = pnet::datalink::interfaces();

    for iface in interfaces {
        for ip in iface.ips {
            if let IpAddr::V6(ipv6) = ip.ip() {
                if !ipv6.is_loopback() && !ipv6.is_unicast_link_local() {
                    ips.push(ipv6.to_string());
                }
            }
        }
    }
    return ips;
}

fn is_port_open(ip: &IpAddr, port: u16) -> bool {
    let addr = format!("[{}]:{}", ip, port);
    TcpStream::connect_timeout(&addr.parse().unwrap(), Duration::from_secs(3)).is_ok()
}
