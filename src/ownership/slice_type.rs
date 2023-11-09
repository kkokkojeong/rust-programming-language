fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

pub fn slice_type() {
    println!("-----slice type-----");
    let mut s = String::from("hello world");

    let word = first_word(&s);

    // ^^^^^^^^^ mutable borrow occurs here
    // s.clear();

    println!("before word: {}", word);
    //
    // s.clear();
    //
    // println!("after word: {}", word);

    // let s = String::from("hello world");
    //
    // let hello = &s[0..5];
    // let world = &s[6..11];
    //
    // println!("hello: {}", hello);
    // println!("world: {}", world);
    //
    // println!("s: {}", s);


    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);

    println!("a: {}", a[0]);
    println!("slice: {}", slice[0]);

}