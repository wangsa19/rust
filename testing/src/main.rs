fn main() {
    println!("Hello, world!");

    biodata("Wangsa", 20);

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    // use function with return value
    let x = seven();
    println!("The value of x is: {x}");
}

fn biodata(name: &str, age: u8) {
    println!("{} is {} years old", name, age);
}

fn seven() -> i32 {
    7
}
