pub fn tuple() {
    let tup: (i32, f64, u8) = (500, 6.1, 1);
    let (_x, y, _z) = tup;

    println!("The value of y is {y}");


    // tuple 접근하는 방법, x.0 이런식으로 접근
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    println!("{five_hundred}, {six_point_four}, {one}");

    // array
    let a = [3; 5]; // let a = [3, 3, 3, 3, 3]


}