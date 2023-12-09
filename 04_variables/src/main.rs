const Z: i32 = 5;

fn main() {
    // variables are immutable by default
    println!("Immutable variables");
    let x = 5;
    println!("x is an immutable variable with value: {x}");
    // x = 6; // error: cannot assign twice to immutable variable `x`
    println!("x's value can not be changed\n");

    // variables can be made mutable with the `mut` keyword
    println!("Mutable variables");
    let mut y = 5;
    println!("y is a mutable variable with value: {y}");
    y = 6;
    println!("y's value has been changed to: {y}\n");

    // variables can be also shadowed
    println!("Shadowing");
    let x = "pi";
    println!("x is a shadowed variable with value: {x}");
    {
        // variables can be shadowed in inner scopes
        let x = "e";
        println!("in this scope x is a shadowed variable with value: {x}");
    }
    println!("in this scope x is a shadowed variable with value: {x}\n");

    // constants are always immutable
    println!("Constants");
    println!("Z is a constant with value: {Z}");
    // Z = 6; // error: cannot assign to this expression
    println!("Z's value can not be changed");
    print!("constants can be declared in any scope, including the global scope\n");
}
