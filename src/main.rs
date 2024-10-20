use std::io::stdin;

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

fn main() {
    let user_name: String = utils::get_full_name("kai", "zeno");
    println!("My Name is {0}", user_name);

    conditionals();
}
