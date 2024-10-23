use std::io::stdin;

use closures::test_closures;
use match_expressions::match_pattern;

pub mod closures;
pub mod match_expressions;

mod utils;

#[allow(dead_code)]
fn data_types() {
    let a: f64 = 64.5;
    let b: i8 = 10;
    let c: f64 = a + (b as f64);
    println!("{}", c);

    let mut is_age_ten: bool = true;
    is_age_ten = false;
    println!("{}", is_age_ten);

    let my_char: char = 'a';
    let my_str: &str = "kai";
    println!("{} {}", my_char, my_str);

    let character: (&str, i8, f64) = ("kai", 2, 3.5);
    println!("{:?}", character);

    let ages: [i8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
    println!("{:?}", ages[0]);

    let slices: &[i8] = &ages[0..=6];
    println!("{:?}", slices);
}

#[allow(dead_code)]
fn conditionals() {
    let age_to_drive: u8 = 16u8;

    println!("Enter your age: ");
    let my_input: &mut String = &mut String::from("");
    std::io::stdin().read_line(my_input).unwrap();

    let age: u8 = my_input.trim().parse::<u8>().unwrap();

    if age >= age_to_drive {
        println!("You can drive");
    } else {
        println!("You cannot drive");
    }
}

#[allow(dead_code)]
fn test_while() {
    let age_to_drive: u8 = 20u8;
    let mut current_age: u8 = 15u8;

    while current_age < age_to_drive {
        println!("Hello");
        current_age += 1;
    }
}

#[allow(dead_code)]
fn test_loop() {
    let mut x: i32 = 0;
    loop {
        println!("hello from rust");
        if x > 5 {
            break;
        }
        x += 1;
    }
}

fn test_for() {
    let ages: [i32; 5] = [8, 18, 20, 30, 40];
    let age_to_drive: i32 = 16;

    for x in ages {
        if x > age_to_drive {
            println!("current age is {0}", x);
        }
    }
}

fn main() {
    // let user_name: String = utils::get_full_name("kai", "zeno");
    // println!("My Name is {0}", user_name);
    // conditionals();
    // test_while();
    // test_loop();
    // test_for();

    // test_closures();

    match_pattern();
}
