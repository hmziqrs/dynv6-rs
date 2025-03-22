use std::collections::HashMap;
use std::io::Error;
use std::net::{IpAddr, TcpStream};
use std::time::Duration;

fn main() -> Result<(), Error> {
    let ip_v6s = get_ipv6s();
    let ports: Vec<u16> = vec![8000, 25565, 19132]; // Change this to the ports you want to check
    let port_to_subdomain: HashMap<u16, String> =
        HashMap::from([(25565, "java6".to_string()), (19132, "brock6".to_string())]);

    let opened_ports = get_opened_ports(ip_v6s, ports);

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

fn get_opened_ports(ips: Vec<String>, ports: Vec<u16>) -> Vec<u16> {
    let mut opened_ports = Vec::<u16>::new();

    for ip in &ips {
        println!("IPv6 Address: {}", ip);
        let ip_addr: IpAddr = ip.parse().unwrap();

        for port in &ports {
            println!("Checking port {} on {}", port, ip);
            if is_port_open(&ip_addr, *port) {
                opened_ports.push(*port);
                println!("Port {} is open on {}", port, ip);
            } else {
                println!("Port {} is closed on {}", port, ip);
            }
        }
    }

    return opened_ports;
}
