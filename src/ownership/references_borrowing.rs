
fn calculate_length(s: &String) -> usize {

    // 만약 수정할려고 하면 아래 컴파일 에러 발생
    //  ^ `s` is a `&` reference, so the data it refers to cannot be borrowed as mutable
    // s.push_str(", world");

    // 만약 mut 를 넣으면 참조 가능
    // let mut s = String::from("hello");

    return s.len();
}

pub fn references() {
    println!("-----references-----");

    let s1 = String::from("hello!");
    let len = calculate_length(&s1);

    // 참조로 넘기지 않으면 s1 를 재사용할 때 아래 컴파일 에러 발생
    // this parameter takes ownership of the value
    // 소유권은 넘기지 않고 참조만 넘김? 용어자체가 어렵네..

    // println!("The length is {}.", len);
    println!("The length of '{}' is {}.", s1, len);

    let mut s2 = String::from("hello");

    // cannot borrow `s2` as mutable more than once at a time
    let r1 = &mut s2;
    let r2 = &mut s2;

    // println!("{}, {}", r1, r2);
}


// this function's return type contains a borrowed value, but there is no value
// for it to be borrowed from
// fn dangle() -> &String {
//     // dangle 함수가 종료되면 s 가 해제되는데 s 참조를 넘길려고 해서 발생한 문제
//     let s = String::from("hello");
//     &s;
// }
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

pub fn dangling() {
    println!("-----dangling-----");
    let reference_to_nothing = no_dangle();

    println!("reference_to_nothing {}", reference_to_nothing);
}