use std::thread;
use std::time::Duration;

struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}
impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    // as-is
    // if intensity < 25 {
    //     println!(
    //         "Today, do {} pushups",
    //         simulated_expensive_calculation(intensity)
    //     );
    //     println!(
    //         "Next, do {} situps",
    //         simulated_expensive_calculation(intensity)
    //     );
    // } else {
    //     if random_number == 3 {
    //         println!("Take a break today! Remember to stay hydrated.");
    //     } else {
    //         println!(
    //             "Today, run for {} minutes",
    //             simulated_expensive_calculation(intensity)
    //         );
    //     }
    // }

    // to-be

    // let expensive_closure = |num: u32| -> u32 {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };

    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    // let expensive_result =
    //     simulated_expensive_calculation(intensity);

    if intensity < 25 {
        println!(
            "Today, do {} pushups",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated.");
        } else {
            println!(
                "Today, run for {} minutes",
                expensive_result.value(intensity)
            );
        }
    }
}

pub fn closures() {
    println!("-----closures-----");

    // simulated_expensive_calculation(0);

    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );

    // let x = 4;
    //
    // let equal_to_x = |z| z == x;
    //
    // let y = 4;
    //
    // assert!(equal_to_x(y));

    // let x = 4;
    //
    // //error[E0434]: can't capture dynamic environment in a fn item
    // fn equal_to_x(z: i32) -> bool { z == x }
    //
    // let y = 4;
    //
    // assert!(equal_to_x(y));



}