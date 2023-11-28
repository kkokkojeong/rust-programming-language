use std::collections::HashMap;

pub fn hash_maps() {
    println!("------hash_maps------");

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("The score is {}.", score);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // 소유권 개념이 너무 어려운데..?


    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // compile error
    // error[E0382]: borrow of moved value: `field_name`
    // println!("{}", field_name);

    // 업데이트
    // 그냥 같은 키값에 insert
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);


    // 키가 없는 경우에만 키와 값 추가
    // or_insert method 사용
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // 이전 값을 기반으로 값 업데이트
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}