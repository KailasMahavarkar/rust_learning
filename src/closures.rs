#[derive(Debug)]

struct Person {
    first_name: String,
    last_name: String,
}

pub fn test_closures() {
    let add = |x, y| -> i32 {
        println!("Sum is --> {0}", x + y);
        x + y
    };
    let result = add(10, 20);
    let print_result = |x: i32| println!("result is {0}", (result + x));
    print_result(32);

    let mut p1 = Person {
        first_name: "kai".to_string(),
        last_name: "zeno".to_string(),
    };

    let mut change_name = |new_last_name: &str| p1.last_name = new_last_name.to_string();
    change_name("jello");
    change_name("zenosama");
    println!("{:?}", p1);

}
