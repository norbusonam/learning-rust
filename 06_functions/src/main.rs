fn some_function() {
    println!("hello from some_function()");
}

fn add(x: i32, y: i32) -> i32 {
    // standard return
    return x + y;
}

fn sub(x: i32, y: i32) -> i32 {
    // implicit return
    x - y
}

fn print_measurement(measurement: f64, unit: &str) {
    println!("the measurement is {measurement}{unit}");
}

fn main() {
    some_function();
    println!("{} + {} = {}", 1, 2, add(1, 2));
    println!("{} - {} = {}", 1, 2, sub(1, 2));
    print_measurement(1.0, "m");
}
