fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {             // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
    // moves out to the calling
    // function
}

fn takes_and_gives_back(mut a_string: String) -> String { // a_string comes into
    // scope
    // Cannot borrow immutable local variable `a_string` as mutable
    a_string.clear();

    a_string  // a_string is returned and moves out to the calling function
}



pub fn ownership() {
    println!("------ownership------");

    // double free error
    // rust는 s1 -> s2 메모리 복사하면 s1 는 더이상 유효하지 않는 것이라 판단
    // rust 는 얉은 복사 shallow copy 대신 "move" 개념 사용
    // let mut s1 = String::from("hello");
    // let s2 = s1.clone();
    //
    // s1.clear();
    // s1.push_str("world!!");
    //
    // println!("s1 is {}", s1);
    // println!("s2 is {}", s2);

    // Ownership and Functions
    // let s = String::from("hello");
    // takes_ownership(s);
    //
    // // compile error: this parameter takes ownership of the value
    // // println!("s is {}", s);
    //
    // let x = 5;
    // makes_copy(x);
    //
    // // compile ok
    // println!("x is {}", x);

    // Return Values and Scope

    let s1 = gives_ownership();         // gives_ownership moves its return

    let mut s2 = String::from("hello");     // s2 comes into scope

    println!("before s2 : {}", s2);

    let s3 = takes_and_gives_back(s2.clone());

    println!("after s2 : {}", s2);
    println!("after s3 : {}", s3);
}