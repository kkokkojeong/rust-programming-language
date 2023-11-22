pub fn utf8_encoded_text() {
    println!("------utf8_encoded_text------");

    // 스트링 생성 방법

    // &str
    let data = "initial contents";

    // the method also works on a literal directly:
    // let s = "initial contents".to_string();
    // let s = data.to_string();
    let s = String::from("initial contents");

    // 문자열 업데이트 `push_str`
    // let mut s1 = String::from("foo");
    // s1.push_str("bar");

    // 연결 concat
    // let s1 = String::from("Hello, ");
    // let s2 = String::from("world!");
    // // 소유권 문제
    // // 아 소유권 어렵네
    //
    // let s3 = s1.clone() + &s2; // note s1 has been moved here and can no longer be used
    //
    // println!("s1 is {}", s1);
    // println!("s2 is {}", s2);
    // println!("s3 is {}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // format!!!
    // let s = s1 + "-" + &s2 + "-" + &s3;
    let s = format!("{s1}-{s2}-{s3}");

    println!("s is {}", s);

    // 문자열로 인덱싱
    let str = String::from("hello");
    // let h = s1[0];

    // ^^^^^ `String` cannot be indexed by `{integer}` 에러 발생

    // 12 byte 가 아닌 24 bytes 한 글자당 2bytes 사용
    // utf8 인코딩
    let hello = String::from("Здr나나나");
    // let answer = &hello[0];
    let s_sliced = &hello[0..4];

    println!("answer is {}.", s_sliced);
    // 그래서 직접 인덱싱을 허용하지 않음

    // 반복
    // character 형태
    for c in "가나다라마바사".chars() {
        println!("{c}");
    }

    // bytes 형태로
    for b in "가나다라마바사".bytes() {
        println!("{b}");
    }

}
