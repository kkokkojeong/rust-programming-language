struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

// 자신만의 반복 iterator 만들기
struct Counter {
    count: u32,
}
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

pub fn iterators() {
    println!("-----iterators------");

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    let total: i32 = v1.iter().sum();
    println!("total: {}", total);

    // map
    // 요건 자바스크립트랑 비슷하네
    let v2: Vec<i32> = vec![1, 2, 3];
    let v3: Vec<i32> = v2.iter().map(|x| x+1).collect();
    let v3_iter = v3.iter();

    for val in v3_iter {
        println!("Got: {}", val);
    }

    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    println!("{}, {}", in_my_size[0].size, in_my_size[0].style);
    println!("{}, {}", in_my_size[1].size, in_my_size[1].style);


    let mut counter = Counter::new();

    // let c1: Option<u32> = counter.next();
    // c1.

    // println!("{}", counter.next());
}