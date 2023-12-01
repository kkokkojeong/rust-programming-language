
// mod guessing_game;
// mod common_concepts;
// mod ownership;
// mod structs;
// mod enums_pattern_matching;

mod common_collections;
mod generic_traits_lifetimes;

// pub use crate

use std::env;
use std::fs::File;
use std::io::prelude::*;


fn main() {
    println!("The Rust Programming");

    // io
    // let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    //
    // let query = &args[1];
    // let filename = &args[2];
    //
    // println!("Searching for {}", query);
    // println!("In file {}", filename);
    //
    // let mut f = File::open(filename).expect("file not found");
    //
    // let mut contents = String::new();
    // f.read_to_string(&mut contents).expect("Something went wrong reading the file");
    //
    // println!("With text:\n{}", contents);

    // https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html
    // guessing_game::guessing();

    // https://doc.rust-lang.org/stable/book/ch03-00-common-programming-concepts.html
    // common_concepts::tuple();
    // common_concepts::functions();

    // https://doc.rust-lang.org/book/ch03-05-control-flow.html
    // common_concepts::control_flow();

    // https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
    // ownership::ownership();
    // ownership::references();
    // ownership::dangling();
    // ownership::slice_type();

    // https://doc.rust-lang.org/stable/book/ch05-01-defining-structs.html#defining-and-instantiating-structs
    // structs::structs_example();
    // structs::method_syntax();

    // enums_pattern_matching::match_control();
    // enums_pattern_matching::if_control_flow();

    // common_collections::list_vectors();
    // common_collections::utf8_encoded_text();
    // common_collections::hash_maps();

    // generic_traits_lifetimes::traits();
    generic_traits_lifetimes::lifetimes();

}



