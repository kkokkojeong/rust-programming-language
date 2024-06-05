
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

// 열거형에도 impl 가능
impl Message {
    pub fn call(&self) {
        println!("call!");
    }
}

// 템플릿 형태도 사용가능
enum Option<T> {
    None,
    Some(T),
}

// rust 에서는 null 개념이 없음
// 10억짜리 실수 재밌네.
// ref: https://doc.rust-lang.org/std/option/enum.Option.html

pub fn enum_ex() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let home1 = IpAddr1::V4(127, 0, 0, 1);
    let loopback1 = IpAddr1::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();
}