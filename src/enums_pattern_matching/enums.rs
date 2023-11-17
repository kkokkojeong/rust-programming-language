// TypeScript Enum 과 동일
enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr1 {
    // V4(String),
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// 템플릿 형태도 사용가능
enum Option<T> {
    None,
    Some(T),
}

pub fn enum_ex() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let home1 = IpAddr1::V4(127, 0, 0, 1);
    let loopback1 = IpAddr1::V6(String::from("::1"));
}