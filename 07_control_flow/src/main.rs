fn compare(x: i32, y: i32) {
    if x < y {
        println!("{x} is less than {y}");
    } else if x > y {
        println!("{x} is greater than {y}");
    } else {
        println!("{x} is equal to {y}");
    }
}

fn multi_print(string: &str, times: i32) {
    for _ in 0..times {
        println!("{}", string);
    }
}

fn main() {
    compare(5, 10);
    compare(10, 5);
    compare(5, 5);

    let x = 5;
    let is_x_even = if x % 2 == 0 { "even" } else { "odd" };
    println!("{} is {}", x, is_x_even);

    multi_print("Hello", 3);

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result: {}", result);
    println!("counter: {}", counter);

    // labelled loop
    'outer: loop {
        println!("entered the outer loop");
        loop {
            println!("entered the inner loop");
            break 'outer;
        }
        // this point will never be reached
    }
    println!("exited the outer loop");

    let mut countdown = 3;
    while countdown != 0 {
        println!("{}...", countdown);
        countdown -= 1;
    }
    println!("liftoff!");

    for number in (1..4).rev() {
        println!("{}...", number);
    }
    println!("liftoff!");

    let a = [10, 20, 30, 40, 50];
    for elem in a {
        print!("{} ", elem);
    }
    println!();
}
