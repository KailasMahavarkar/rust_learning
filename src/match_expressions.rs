fn integer_match() {
    let my_age: u16 = 240;

    match my_age {
        0 => println!("your age is 0"),
        1..=30 => println!("you are young"),
        31..60 => println!("you are old"),
        60.. => println!("you lived happy"),
    }
}

fn match_guard() {
    let my_age: u32 = 10000;
    let y: u32 = 110;
    match my_age {
        0.. if y == 10 => println!("0.. & y is 10"),
        0.. if y != 10 => println!("0.. & y is not 10"),
        0.. => println!("defaults"),
    }
}

fn string_match() {
    let car_names = "porsche";

    match car_names {
        "porsche" => {
            println!("it is porsche");
        }
        _ => {
            println!("not a porsche");
        }
    }
}

fn string_match_obj() -> String {
    let carnames = "porsche";

    let selected = match carnames {
        "hyundai" => "hyundai",
        "porsche" => "porsche",
        _ => "others",
    }
    .to_string();

    println!("{}", selected);
    return selected;
}

pub fn test_match_slice() {
    let ages: [u32; 8] = [0, 1, 2, 3, 4, 5, 6, 7];

    match ages[0..=2] {
        [0, 1, 2] => println!("between 0 to 2"),
        _ => println!("defaults"),
    }
}

pub fn match_pattern() {
    integer_match();
    match_guard();
    string_match();
    string_match_obj();

    test_match_slice();
}
