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
    Quarter(UsState),
}

pub fn if_control_flow() {
    println!("-----if_control_flow-----");

    let config_max = Some(5u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // if vs match
    // if 는 간결하지만 모든 케이스를 커버하지 않음
    // syntax sugar

    let coin = Coin::Quarter(UsState::Alaska);
    // let coin = Coin::Penny;

    let mut count = 0;
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    //     _ => count += 1,
    // }

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

    println!("The count is {}.", count);
}