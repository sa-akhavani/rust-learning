#[derive(Debug)]
enum IpAddressKind {
    V4(String),
    V6(String),
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddressKind,
    address: String,
}

#[derive(Debug)]
enum IpAddrV2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let ipv4 = IpAddressKind::V4(String::from("192.168.0.1"));
    let ipv6 = IpAddressKind::V6(String::from("::1"));
    println!("v4: {:?}\r\nv6: {:?}", ipv4, ipv6);
    print_ip_kind(ipv4);

    // let localhost = IpAddr {
    //     kind: IpAddressKind::V4,
    //     address: String::from("192.168.0.1"),
    // };
    //
    // let loopback = IpAddr {
    //     kind: IpAddressKind::V6,
    //     address: String::from("::1"),
    // };
    let localhost = IpAddressKind::V4(String::from("192.168.0.1"));
    let loopback = IpAddressKind::V6(String::from("::1"));

    println!("{:?}", localhost);
    println!("{:?}", loopback);

    let localhost = IpAddrV2::V4(192, 168, 0, 1);
    let loopback = IpAddrV2::V6(String::from("::1"));
    println!("{:?}", localhost);
    println!("{:?}", loopback);
}

fn print_ip_kind(ip: IpAddressKind) {
    println!("{:?}", ip);
}
