fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);

    This will lead to an error!
    x = 6;
    println!("The value of x is: {}", x);

    let x = "This will work";
    println!("The value of x is: {}", x);

    // 1. This will generate error
    // 2. Shadowing vs Mutation
    // 3. `mut` only allows to mutate to the same type!
    // 4. Shadowing is like redefining a variable 
    // x = 6;
}
