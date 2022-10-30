#![allow(unused)]

use core::num;
use std::{io, ops::Add};
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    some_tuples();
    //some_loops();
    //some_types();
    //greetings();
    // strings();
}



fn some_tuples() {
    let my_tuple: (u8, String, f64) = (128, "A Secret Message".to_string(), 100_000.00);
    println!("what's in the bottle? {}", my_tuple.1);
    let (v1, v2, v3) = my_tuple;
}

fn some_loops() {
    let numbers = [1,2,3,4,5,6,7,8,9];
    let even = numbers
        .into_iter().filter(|x| {x % 2 == 0})
        .for_each(|x| println!("an even number {}", x));

    for elem in numbers {
        println!("number {}", elem);
    }
}

fn some_types() {
    println!("Max value of u32 is {}", u32::MAX);
    println!("Max value of u64 is {}", u64::MAX);
    println!("Max value of usize is {}", usize::MAX);

    let f_1: f32 = 1.111111111111111;
    println!("f32 : {}", f_1 + 0.111111111111111);

    let f_2: f64 = 1.111111111111111;
    println!("f64 : {}", f_2 + 0.111111111111111);


    let random_num = rand::thread_rng().gen_range(1..100);
    println!("random number is {}", random_num);

    let my_age = 14;
    let voting_age = 18;
    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("Can't vote"),
        _ => println!("Can vote")
    };

}

fn greetings() {
    let mut name = String::new();
    let greetings = "Greetings";
    println!("What's your name? ");
    io::stdin().read_line(&mut name)
        .expect("Didn't recive input.");
    println!("{}, {}!", greetings, name.trim_end());
}

fn strings() {
    let hello = "Hello!".to_string();
    let bye = "bye";
    let good = String::from("Good");
    let good_bye = good.add(" bye!");
    println!("{}", hello);
    println!("{}", bye);
    println!("{}", good_bye);

    let mut owned_string = "Hello".to_owned();
    let borrowed_string = " World!";
    owned_string.push_str(borrowed_string);
    println!("{}", owned_string);

}