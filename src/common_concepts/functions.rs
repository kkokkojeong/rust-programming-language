fn five() -> i32 {
    // return 을 안 적어줘도 되네?
    5
}

pub fn functions() {
    // Rust code uses snake case as the conventional style for function and variable names
    let x = five();
    println!("The value of x is:  {x}");
}