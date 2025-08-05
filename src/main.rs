enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    addr: [u8; 4],
}

fn main() {
    let addr = IpAddr {
        kind: IpAddrKind::V4,
        addr: [127, 0, 0, 1],
    };

    let ip_version = match addr.kind {
        IpAddrKind::V4 => "ipv4",
        IpAddrKind::V6 => "ipv6",
    };
    println!("IP version: {ip_version}");

    let mut addr_str = String::new();
    for (i, part) in addr.addr.iter().enumerate() {
        let part = part.to_string();
        addr_str.push_str(part.as_str());

        if i != addr.addr.iter().len() - 1 {
            addr_str.push_str(".");
        }
    }

    println!("IP address: {addr_str}");
}
