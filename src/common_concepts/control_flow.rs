pub fn control_flow() {
    // let number = 3;
    //
    // if number < 5 {
    //     println!("condition was true");
    // } else {
    //     println!("condition was false");
    // }
    //
    // let number = 6;
    //
    // if number % 4 == 0 {
    //     println!("number is divisible by 4");
    // } else if number % 3 == 0 {
    //     println!("number is divisible by 3");
    // } else if number % 2 == 0 {
    //     println!("number is divisible by 2");
    // } else {
    //     println!("number is not divisible by 4, 3, or 2");
    // }
    //
    // let condition = true;
    // // 이렇게도 할당을 하는구나...
    // let number = if condition { 5 } else { 6 };
    //
    // println!("number is: {number}");
    //
    // // Returning Values from Loops
    // let mut counter = 0;
    //
    // let result = loop {
    //     counter += 1;
    //
    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };
    //
    // println!("The result is {result}");

    // Loop Labels to Disambiguate Between Multiple Loops
    // 외부 loop 를 종료 시키는 법 break + label
    // let mut count = 0;
    // 'disambiguate: loop {
    //     println!("count = {count}");
    //     let mut remaining = 10;
    //
    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'disambiguate;
    //         }
    //         remaining -= 1;
    //     }
    //
    //     count += 1;
    // }
    // println!("End count = {count}");

    // array for in
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }

    println!("LIFTOFF!!!");
}