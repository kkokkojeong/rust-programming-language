#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    // 값에 바인딩되는 패턴
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    // 신기한 문법이네
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // 모든 케이스에 대해 처리해야함
    // ensure that all possible cases are being handled
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

pub fn match_control() {
    println!("-----match_control-----");

    // println!("match is {}", value_in_cents(Coin::Penny));
    value_in_cents(Coin::Quarter(UsState::Alaska));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("six is {:?}.", six);
    println!("none is {:?}.", none);
}