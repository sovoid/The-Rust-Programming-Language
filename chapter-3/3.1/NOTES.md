## 3.1 Variables and Mutability

- by default, variables are immutable

```rust
    let x = 5;
    println!("The value of x is: {}", x); // prints 5

    x = 6;
    println!("The value of x is: {}", x) // error
```

- this prevents hard-to-track bugs

- specify the `mut` keyword for mutating

```rust
    let mut x = 5;
    println!("The value of x is: {}", x); //prints 5

    x = 6;
    println!("The value of x is: {}", x); // prints 6
```

* `const` are different from `let`

| **const**            | **let**                  |
| -------------------- | ------------------------ |
| need to be annoted   | no annotation compulsory |
| cannot use shadowing | can be shadowed          |
| cannot use `mut`     | can use `mut`            |

* _shadowing_ allows us to redefine variables

```rust
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Number of spaces: {}", spaces); //prints 3
```

* Type annotation

- Rust is statically, strongly typed
- it can infer types and has defaults.
- we can specify type like:

```rust
let x: u32 = 100;
```