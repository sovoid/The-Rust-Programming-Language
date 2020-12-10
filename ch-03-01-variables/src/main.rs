fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    
    // ! Error: cannot assign twice to immutable variable
    // x = 6;

    // * Declare variable as a mutable
    let mut y = 10;
    println!("The value of y is: {}", y);
    y = 15;
    println!("The value of y is: {}", y);

    // * Constants must be annoted
    const SECONDS_IN_A_DAY: u32 = 876_000;

    // * Shadowing using `let`
    let spaces = "";
    let spaces = "  "; // * Mutate to the same type
    let spaces = spaces.len(); // * Mutate to a different type

    // * Shadowing vs Mutation
    let m = 20;
    let m = "thirty"; // This is allowed

    let mut n = 20;
    // ! Error: cannot mutate variable's type
    // n = "twenty";
}