fn give_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn take_ownership(some_string: String) {
    println!("take_ownership now owns {}", some_string);
}

fn take_ownership_and_return(some_string: String) -> String {
    println!("take_ownership_and_return now owns {}", some_string);
    some_string
}

fn main() {
    // moving ownership w/ assignment
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}, world!", s1); // s2 is the owner of the string now, so this will fail
    println!("{}, world!", s2);

    // moving ownership w/ function
    let s1 = String::from("hello");
    take_ownership(s1);
    // println!("{}, world!", s1); // s1 is the owner of the string now, so this will fail

    // returning ownership
    let s1 = give_ownership();
    println!("{}, world!", s1);

    // moving ownership w/ function and returning ownership
    let s1 = String::from("hello");
    let s2 = take_ownership_and_return(s1);
    // println!("{}, world!", s1); // s2 is the owner of the string now, so this will fail
    println!("{}, world!", s2);

    // cloning
    let s1 = String::from("hello");
    let mut s2 = s1.clone();
    s2.push('!');
    println!("s1 = {}, s2 = {}", s1, s2)
}
