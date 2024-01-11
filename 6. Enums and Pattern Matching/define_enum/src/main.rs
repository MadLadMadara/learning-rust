#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}


fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("four: {:#?}, six: {:#?}", four, six);

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    
}


fn route(ip_kind: IpAddrKind) {
    println!("ip_kind {:#?}", ip_kind);
}
