pub fn list_vectors() {
    println!("------list_vectors------");

    // create
    let mut v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];

    // update
    v1.push(5);
    v1.push(6);

    // indexing
    let v = vec![1, 2, 3, 4, 5];

    // let third: &i32 = &v[2];
    let third = v.get(2);
    // println!("The third element is {third}");
    match third {
        Some(third) => println!("The third element is {:?}", third),
        None => println!("There is no third element."),
    }

    // iterating
    let vv = vec![100, 32, 57];
    for i in &vv {
        println!("{i}");
    }
}