// c언어 구조체 개념
// TypeScript interface 비슷
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// 명명된 필드 없이 튜플 구조체를 사용하여 다양한 유형 만들기
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);


fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

pub fn structs() {
    println!("-----structs-----");

    // 괄호없이 생성하네?
    let mut user1 = User {
        active: false,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 0,
    };

    println!("before email {}", user1.email);

    // Rust에서는 특정 필드만 변경 가능으로 표시하는 것을 허용하지 않습니다
    // mut 사용해야함
    user1.email = String::from("anotheremail@example.com");

    println!("after email {}", user1.email);

    // JavaScript spread 문법이랑 거의 비슷
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("after user2 email is {}", user2.email);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{}", black.0);
}