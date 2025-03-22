use std::io::Error;
use std::net::IpAddr;

fn main() -> Result<(), Error> {
    let interfaces = pnet::datalink::interfaces();

    for iface in interfaces {
        for ip in iface.ips {
            if let IpAddr::V6(ipv6) = ip.ip() {
                if !ipv6.is_loopback() && !ipv6.is_unicast_link_local() {
                    println!("IPv6 Address: {}", ipv6);
                    return Ok(());
                }
            }
        }
    }

    println!("No valid global IPv6 address found.");
    Ok(())
}
