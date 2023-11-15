// println! 으로 입력하기 위해서 입력하는 속성
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn structs_example() {
    println!("-----structs_example------");
    let rect = Rectangle {
        width: 10,
        height: 10
    };


    // {} -> {:?}, {:#?} 형태로 디버깅 출력
    println!("{:?}", rect); // 가로로 출력
    println!("{:#?}", rect); // 줄바꿈

    // 코드 라인까지 출력
    dbg!(&rect);
}