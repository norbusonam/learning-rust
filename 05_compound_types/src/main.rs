fn main() {
    // tuples
    let person = ("John Doe", 42);
    let (name, age) = person;
    println!("{} is {} years old.", name, age);
    println!("{} is {} years old.\n", person.0, person.1);

    // arrays
    let numbers = [1, 2, 3, 4, 5];
    println!("The third number is {}.\n", numbers[2]);
    // println!("The 10th number is {}.\n", numbers[9]); // error: index out of bounds
}
